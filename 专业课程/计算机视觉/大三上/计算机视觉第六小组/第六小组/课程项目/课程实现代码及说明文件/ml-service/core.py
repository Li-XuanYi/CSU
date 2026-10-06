#!/usr/bin/env python3
"""
Shared ML utilities for runners/services.
"""

from __future__ import annotations

import logging
import os
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path
from typing import Iterable, List, Optional

import torch
from PIL import Image
from transformers import AutoModel, AutoProcessor

try:
    from dotenv import load_dotenv

    load_dotenv()
except ModuleNotFoundError:
    pass


def get_device() -> str:
    return "cuda" if torch.cuda.is_available() else "cpu"


def resolve_base_dir() -> Path:
    return Path(
        os.getenv("PROJECT_ROOT")
        or os.getenv("UPLOAD_BASE_DIR")
        or Path(__file__).resolve().parents[1]
    )


def get_env_bool(key: str, default: bool = False) -> bool:
    raw = os.getenv(key)
    if raw is None:
        return default
    return raw.strip() not in {"0", "false", "False", "no", "NO"}


def get_env_list(key: str, default: Iterable[str]) -> List[str]:
    raw = os.getenv(key)
    if raw is None:
        return list(default)
    return [item.strip() for item in raw.split(",") if item.strip()]


def post_json(url: str, payload: dict, headers: dict, timeout: int = 30) -> None:
    import requests

    resp = requests.post(url, json=payload, headers=headers, timeout=timeout)
    if resp.status_code >= 300:
        raise RuntimeError(f"Callback failed: {resp.status_code} {resp.text}")


@dataclass
class ClipEmbedder:
    model_name: str
    expected_dim: int = 0
    device: Optional[str] = None

    def __post_init__(self) -> None:
        self.device = self.device or get_device()
        self.processor = AutoProcessor.from_pretrained(
            self.model_name, trust_remote_code=True
        )
        self.model = self._load_model()
        self.model.eval()

    def _load_model(self):
        dtype = torch.float16 if self.device == "cuda" else None
        for attempt in ("default", "no_safetensors"):
            try:
                model = AutoModel.from_pretrained(
                    self.model_name,
                    low_cpu_mem_usage=False,
                    trust_remote_code=True,
                    _fast_init=False,
                    dtype=dtype,
                    use_safetensors=False if attempt == "no_safetensors" else None,
                )
                if any(getattr(p, "is_meta", False) for p in model.parameters()):
                    raise RuntimeError("Model initialized on meta tensors")
                return model.to(self.device)
            except Exception as exc:
                logging.warning("Model load attempt %s failed (%s)", attempt, exc)

        raise RuntimeError(
            f"Failed to load model on {self.device}. "
            "Please check CUDA/torch versions or try installing accelerate."
        )

    def embed_text(self, text: str) -> List[float]:
        inputs = self.processor(text=[text], return_tensors="pt", padding=True)
        inputs = {k: v.to(self.device) for k, v in inputs.items()}
        with torch.no_grad():
            feats = None
            if hasattr(self.model, "get_text_features"):
                try:
                    feats = self.model.get_text_features(**inputs)
                except Exception:
                    feats = None
            if feats is None:
                if hasattr(self.model, "text_model"):
                    text_model = self.model.text_model
                    text_outputs = text_model(
                        input_ids=inputs.get("input_ids"),
                        attention_mask=inputs.get("attention_mask"),
                        token_type_ids=inputs.get("token_type_ids"),
                    )
                    pooled = getattr(text_outputs, "pooler_output", None)
                    if pooled is None:
                        pooled = text_outputs.last_hidden_state[:, 0, :]
                    projection = getattr(self.model, "text_projection", None)
                    feats = projection(pooled) if projection is not None else pooled
                else:
                    outputs = self.model(**inputs)
                    feats = getattr(outputs, "text_embeds", None)
                if feats is None:
                    raise RuntimeError("Model does not provide text embeddings")
            feats = feats / feats.norm(p=2, dim=-1, keepdim=True)
        vec = feats.squeeze(0).cpu().tolist()
        self._check_dim(vec)
        return vec

    def embed_image(self, image: Image.Image) -> List[float]:
        inputs = self.processor(images=image, return_tensors="pt")
        inputs = {k: v.to(self.device) for k, v in inputs.items()}
        with torch.no_grad():
            feats = None
            if hasattr(self.model, "get_image_features"):
                try:
                    feats = self.model.get_image_features(**inputs)
                except Exception:
                    feats = None
            if feats is None:
                if hasattr(self.model, "vision_model"):
                    vision_model = self.model.vision_model
                    vision_outputs = vision_model(pixel_values=inputs.get("pixel_values"))
                    pooled = getattr(vision_outputs, "pooler_output", None)
                    if pooled is None:
                        pooled = vision_outputs.last_hidden_state[:, 0, :]
                    projection = getattr(self.model, "visual_projection", None)
                    feats = projection(pooled) if projection is not None else pooled
                else:
                    outputs = self.model(**inputs)
                    feats = getattr(outputs, "image_embeds", None)
                if feats is None:
                    raise RuntimeError("Model does not provide image embeddings")
            feats = feats / feats.norm(p=2, dim=-1, keepdim=True)
        vec = feats.squeeze(0).cpu().tolist()
        self._check_dim(vec)
        return vec

    def _check_dim(self, vec: List[float]) -> None:
        if self.expected_dim and len(vec) != self.expected_dim:
            logging.warning(
                "Embedding dim mismatch: expected %s, got %s",
                self.expected_dim,
                len(vec),
            )


@lru_cache(maxsize=4)
def get_clip_embedder(model_name: str, expected_dim: int = 0) -> ClipEmbedder:
    return ClipEmbedder(model_name=model_name, expected_dim=expected_dim)


def read_image_rgb(path: Path) -> Image.Image:
    return Image.open(path).convert("RGB")


@dataclass
class FaceEmbedder:
    model_name: str
    device: Optional[str] = None

    def __post_init__(self) -> None:
        from insightface.app import FaceAnalysis

        self.device = self.device or get_device()
        providers = ["CUDAExecutionProvider", "CPUExecutionProvider"]
        self.face_app = FaceAnalysis(name=self.model_name, providers=providers)
        ctx_id = 0 if self.device == "cuda" else -1
        self.face_app.prepare(ctx_id=ctx_id, det_size=(640, 640))

    def extract_faces(self, image_bgr) -> List[dict]:
        faces = self.face_app.get(image_bgr)
        results = []
        for face in faces:
            emb = getattr(face, "normed_embedding", None)
            if emb is None:
                emb = getattr(face, "embedding", None)
            if emb is None:
                continue
            bbox = getattr(face, "bbox", None)
            bbox_dict = None
            if bbox is not None:
                bbox_dict = {
                    "x1": float(bbox[0]),
                    "y1": float(bbox[1]),
                    "x2": float(bbox[2]),
                    "y2": float(bbox[3]),
                }
            results.append({"embedding": emb.tolist(), "bbox": bbox_dict})
        return results
