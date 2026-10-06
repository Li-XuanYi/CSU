#!/usr/bin/env python3
"""
简易 CLIP runner：
- 从 Redis 队列（默认 photo_tasks）阻塞消费任务
- 读取任务中的图片路径，使用 HuggingFace CLIP 生成 embedding
- 调用后端内部接口 /api/internal/photos/{id}/ml_result 写回 embedding

环境变量（可用 .env 配置）：
- REDIS_URL: Redis 连接串，默认 redis://127.0.0.1:6379/0
- REDIS_QUEUE_NAME: 任务队列名，默认 photo_tasks
- REDIS_DLQ_NAME: 失败时的死信队列名，默认 photo_tasks_dlq
- API_BASE_URL: 服务地址，默认 http://127.0.0.1:5800
- PROCESSING_INTERNAL_TOKEN: 与服务端配置一致的内部 token
- CLIP_MODEL_NAME: HuggingFace 模型名，默认 OFA-Sys/chinese-clip-vit-large-patch14
- PROCESSING_EMBEDDING_DIM: 预期 embedding 维度，默认 768
- PROCESSING_DEFAULT_MODEL: 写回数据库时记录的 model 名称，默认 config 中的 chinese-clip-vit-large-patch14
- OCR_ENABLED: 是否启用 OCR，默认 1
- OCR_LANGS: OCR 语言列表，逗号分隔，默认 ch_sim,en
- OCR_MIN_CONF: 置信度阈值，默认 0.3
- OCR_MAX_CHARS: 写回最大字符数，默认 2000
"""

import json
import logging
import os
import time
from dataclasses import dataclass
from typing import Optional

from pathlib import Path

import redis
import torch
import cv2
import numpy as np
from PIL import Image

from core import (
    ClipEmbedder,
    FaceEmbedder,
    get_device,
    get_env_bool,
    get_env_list,
    post_json,
    resolve_base_dir,
)

try:
    import easyocr
except ModuleNotFoundError:
    easyocr = None
try:
    from transformers import BlipForConditionalGeneration, BlipProcessor
except ModuleNotFoundError:
    BlipForConditionalGeneration = None
    BlipProcessor = None

logging.basicConfig(
    level=logging.INFO,
    format="[%(asctime)s] %(levelname)s %(message)s",
)


@dataclass
class Task:
    photo_id: int
    user_id: int
    path: str
    tasks: list

    @classmethod
    def from_raw(cls, raw: str) -> "Task":
        data = json.loads(raw)
        return cls(
            photo_id=int(data["photo_id"]),
            user_id=int(data["user_id"]),
            path=data["path"],
            tasks=data.get("tasks", []),
        )


class ClipRunner:
    def __init__(self) -> None:
        self.redis_url = os.getenv("REDIS_URL", "redis://127.0.0.1:6379/0")
        self.queue_name = os.getenv("REDIS_QUEUE_NAME", "photo_tasks")
        self.dlq_name = os.getenv("REDIS_DLQ_NAME", f"{self.queue_name}_dlq")
        self.api_base = os.getenv("API_BASE_URL", "http://127.0.0.1:5800")
        self.internal_token = os.getenv("PROCESSING_INTERNAL_TOKEN", "change-me")
        self.embedding_dim = int(os.getenv("PROCESSING_EMBEDDING_DIM", "768"))
        self.device = get_device()
        logging.info("Using device: %s", self.device)
        self.models = self._load_models()

        # Face embedding settings
        self.face_model_name = os.getenv("FACE_MODEL_NAME", "buffalo_l")
        self.face_embedding_dim = int(os.getenv("PROCESSING_FACE_EMBEDDING_DIM", "512"))
        self.default_face_model_label = os.getenv(
            "PROCESSING_DEFAULT_FACE_MODEL", "arcface-buffalo_l"
        )
        self.caption_enabled = get_env_bool("CAPTION_ENABLED", False)
        self.caption_model_name = os.getenv(
            "CAPTION_MODEL_NAME", "Salesforce/blip-image-captioning-base"
        )
        self.caption_max_length = int(os.getenv("CAPTION_MAX_LENGTH", "64"))
        self.caption_processor = None
        self.caption_model = None
        self.ocr_enabled = get_env_bool("OCR_ENABLED", True)
        self.ocr_langs = get_env_list("OCR_LANGS", ["ch_sim", "en"])
        self.ocr_min_conf = float(os.getenv("OCR_MIN_CONF", "0.3"))
        self.ocr_max_chars = int(os.getenv("OCR_MAX_CHARS", "2000"))
        self.ocr_reader = None

        # 解决任务里的相对路径：默认取仓库根目录（ml 的上级），或使用 PROJECT_ROOT/UPLOAD_BASE_DIR 覆盖
        self.base_dir = resolve_base_dir()

        # 预加载模型，避免每个任务重复开销
        for m in self.models:
            logging.info("Loaded CLIP model label=%s hf=%s", m["label"], m["name"])

        if self.caption_enabled:
            if BlipProcessor is None or BlipForConditionalGeneration is None:
                logging.warning("BLIP not available, caption disabled")
                self.caption_enabled = False
            else:
                self.caption_processor = BlipProcessor.from_pretrained(
                    self.caption_model_name
                )
                self.caption_model = BlipForConditionalGeneration.from_pretrained(
                    self.caption_model_name
                ).to(self.device)
                self.caption_model.eval()
                logging.info("Loaded caption model %s", self.caption_model_name)

        if self.ocr_enabled:
            if easyocr is None:
                logging.warning("easyocr not installed, OCR disabled")
                self.ocr_enabled = False
            else:
                self.ocr_reader = easyocr.Reader(
                    self.ocr_langs, gpu=torch.cuda.is_available()
                )
                logging.info("Loaded OCR reader langs=%s", ",".join(self.ocr_langs))

        # 人脸模型（insightface），优先 GPU
        self.face_embedder = FaceEmbedder(self.face_model_name, device=self.device)

        self.redis = redis.from_url(self.redis_url)

    def run_forever(self) -> None:
        logging.info("Runner started. Waiting for tasks on %s", self.queue_name)
        while True:
            task = self._pop_task()
            if not task:
                continue

            try:
                self._process_task(task)
            except Exception:
                logging.exception("Task %s failed", task)
                self._push_dlq(task)
                # 避免疯狂重试，稍作等待
                time.sleep(1)

    def _pop_task(self) -> Optional[Task]:
        item = self.redis.brpop(self.queue_name, timeout=5)
        if item is None:
            return None
        _, payload = item
        try:
            return Task.from_raw(payload.decode("utf-8"))
        except Exception:
            logging.exception("Failed to parse task payload: %s", payload)
            return None

    def _push_dlq(self, task: Task) -> None:
        try:
            self.redis.lpush(self.dlq_name, json.dumps(task.__dict__))
        except Exception:
            logging.exception("Failed to push task to DLQ")

    def _process_task(self, task: Task) -> None:
        img_path = self._resolve_path(task.path)
        if not img_path.exists():
            raise FileNotFoundError(f"Image not found: {img_path}")

        logging.info("Processing photo %s for user %s", task.photo_id, task.user_id)

        if "clip" in task.tasks:
            self._run_clip(task.photo_id, img_path)
            if self.caption_enabled:
                self._run_caption(task.photo_id, img_path)

        if "face" in task.tasks or "faces" in task.tasks:
            self._run_faces(task.photo_id, img_path)

        if "ocr" in task.tasks:
            self._run_ocr(task.photo_id, img_path)

    def _run_clip(self, photo_id: int, img_path: Path) -> None:
        image = Image.open(img_path).convert("RGB")
        for m in self.models:
            embedding = m["embedder"].embed_image(image)

            payload = {
                "model": m["label"],
                "modality": "image",
                "embedding": embedding,
                "ml_result": {
                    "source": "clip_runner",
                    "model_name": m["name"],
                    "device": self.device,
                },
                "processing_status": "done",
            }

            url = f"{self.api_base}/api/internal/photos/{photo_id}/ml_result"
            headers = {"x-internal-token": self.internal_token}
            post_json(url, payload, headers)

            logging.info(
                "Photo %s: clip embedding updated (model=%s)", photo_id, m["label"]
            )

    def _run_faces(self, photo_id: int, img_path: Path) -> None:
        img = cv2.imread(str(img_path))
        if img is None:
            raise RuntimeError(f"Failed to read image for faces: {img_path}")

        faces = self.face_embedder.extract_faces(img)
        if not faces:
            logging.info("Photo %s: no faces detected", photo_id)
            url = f"{self.api_base}/api/internal/photos/{photo_id}/faces"
            headers = {"x-internal-token": self.internal_token}
            post_json(url, {"faces": []}, headers)
            return

        payload_faces = []
        for face in faces:
            emb_list = face["embedding"]
            if len(emb_list) != self.face_embedding_dim:
                logging.warning(
                    "Face embedding dim mismatch: expected %s, got %s",
                    self.face_embedding_dim,
                    len(emb_list),
                )

            payload_faces.append(
                {
                    "embedding": emb_list,
                    "model": self.default_face_model_label,
                    "bbox": face["bbox"],
                }
            )

        if not payload_faces:
            logging.info("Photo %s: faces detected but no embeddings", photo_id)
            return

        url = f"{self.api_base}/api/internal/photos/{photo_id}/faces"
        headers = {"x-internal-token": self.internal_token}
        post_json(url, {"faces": payload_faces}, headers)

        logging.info(
            "Photo %s: %s faces uploaded (model=%s)",
            photo_id,
            len(payload_faces),
            self.default_face_model_label,
        )

    def _run_caption(self, photo_id: int, img_path: Path) -> None:
        if not self.caption_processor or not self.caption_model or not self.models:
            return

        image = Image.open(img_path).convert("RGB")
        inputs = self.caption_processor(images=image, return_tensors="pt").to(self.device)
        with torch.no_grad():
            out = self.caption_model.generate(
                **inputs, max_new_tokens=self.caption_max_length
            )
        caption = self.caption_processor.decode(out[0], skip_special_tokens=True).strip()
        if not caption:
            logging.info("Photo %s: caption empty, skip caption embedding", photo_id)
            return

        # 复用首个 CLIP 模型编码 caption
        embedding = self.models[0]["embedder"].embed_text(caption)

        payload = {
            "model": self.models[0]["label"],
            "modality": "caption",
            "embedding": embedding,
            "processing_status": "done",
        }

        url = f"{self.api_base}/api/internal/photos/{photo_id}/ml_result"
        headers = {"x-internal-token": self.internal_token}
        post_json(url, payload, headers)

        logging.info("Photo %s: caption='%s' embedding updated", photo_id, caption)

    def _run_ocr(self, photo_id: int, img_path: Path) -> None:
        if not self.ocr_enabled or self.ocr_reader is None:
            logging.info("Photo %s: OCR disabled, skip", photo_id)
            return

        img = cv2.imread(str(img_path))
        if img is None:
            raise RuntimeError(f"Failed to read image for OCR: {img_path}")

        results = self.ocr_reader.readtext(img)
        items = []
        texts = []
        for bbox, text, conf in results:
            if conf < self.ocr_min_conf:
                continue
            cleaned = text.strip()
            if not cleaned:
                continue
            texts.append(cleaned)
            items.append(
                {
                    "text": cleaned,
                    "conf": float(conf),
                    "bbox": np.array(bbox).tolist(),
                }
            )

        if not texts:
            logging.info("Photo %s: no OCR text detected", photo_id)
            payload = {
                "ml_result": {"ocr_text": ""},
                "processing_status": "done",
            }
            url = f"{self.api_base}/api/internal/photos/{photo_id}/ml_result"
            headers = {"x-internal-token": self.internal_token}
            post_json(url, payload, headers)
            return

        ocr_text = " ".join(texts)
        if self.ocr_max_chars > 0:
            ocr_text = ocr_text[: self.ocr_max_chars]

        payload = {
            "ml_result": {
                "ocr_text": ocr_text,
                "ocr": {
                    "text": ocr_text,
                    "items": items,
                    "lang": self.ocr_langs,
                    "source": "easyocr",
                },
            },
            "processing_status": "done",
        }

        url = f"{self.api_base}/api/internal/photos/{photo_id}/ml_result"
        headers = {"x-internal-token": self.internal_token}
        post_json(url, payload, headers)

        logging.info("Photo %s: OCR text updated", photo_id)

    def _resolve_path(self, path: str) -> Path:
        candidate = Path(path)
        if candidate.is_absolute():
            return candidate

        combined = self.base_dir / candidate
        if combined.exists():
            return combined

        # 兜底返回绝对化的原路径（便于错误提示）
        return candidate.resolve()

    def _load_models(self):
        # 支持多模型：CLIP_MODELS 格式 label=hf_repo,label2=hf_repo2
        # 例如：clip14=openai/clip-vit-large-patch14,laion=laion/CLIP-ViT-H-14-laion2B-s32B-b79K
        # 若未配置则退回单模型（CLIP_MODEL_NAME + PROCESSING_DEFAULT_MODEL）
        raw = os.getenv("CLIP_MODELS")
        entries = []
        if raw:
            for item in raw.split(","):
                if "=" in item:
                    label, name = item.split("=", 1)
                    entries.append((label.strip(), name.strip()))
        if not entries:
            default_label = os.getenv(
                "PROCESSING_DEFAULT_MODEL", "chinese-clip-vit-large-patch14"
            )
            default_name = os.getenv(
                "CLIP_MODEL_NAME", "OFA-Sys/chinese-clip-vit-large-patch14"
            )
            entries.append((default_label, default_name))

        models = []
        for label, name in entries:
            embedder = ClipEmbedder(
                model_name=name, expected_dim=self.embedding_dim, device=self.device
            )
            models.append({"label": label, "name": name, "embedder": embedder})
        return models
