#!/usr/bin/env python3
"""
文本嵌入 runner（异步队列版）：
- 队列消费 text_tasks（可用 TEXT_EMBED_QUEUE_NAME 配置）
- 任务格式：{"type": "text_embed", "request_id": 123, "query": "some text", "model": "chinese-clip-vit-large-patch14"}
- 生成 CLIP 文本 embedding 后回调内部接口 /api/internal/text_queries/{id}

环境变量：
- REDIS_URL (默认 redis://127.0.0.1:6379/0)
- TEXT_EMBED_QUEUE_NAME (默认 text_tasks)
- TEXT_EMBED_DLQ_NAME (默认 text_tasks_dlq)
- API_BASE_URL (默认 http://127.0.0.1:5800)
- PROCESSING_INTERNAL_TOKEN (后端内部 token)
- CLIP_MODEL_NAME (默认 OFA-Sys/chinese-clip-vit-large-patch14)
- PROCESSING_EMBEDDING_DIM (默认 768)
"""

import json
import logging
import os
import time
from dataclasses import dataclass

import redis

from core import ClipEmbedder, get_device, post_json

logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")


@dataclass
class TextTask:
    request_id: int
    query: str
    model: str

    @classmethod
    def from_raw(cls, raw: str) -> "TextTask":
        data = json.loads(raw)
        return cls(
            request_id=int(data["request_id"]),
            query=data["query"],
            model=data.get("model") or "chinese-clip-vit-large-patch14",
        )


class TextRunner:
    def __init__(self) -> None:
        self.redis_url = os.getenv("REDIS_URL", "redis://127.0.0.1:6379/0")
        self.queue = os.getenv("TEXT_EMBED_QUEUE_NAME", "text_tasks")
        self.dlq = os.getenv("TEXT_EMBED_DLQ_NAME", f"{self.queue}_dlq")
        self.api_base = os.getenv("API_BASE_URL", "http://127.0.0.1:5800")
        self.internal_token = os.getenv("PROCESSING_INTERNAL_TOKEN", "change-me")
        self.model_name = os.getenv(
            "CLIP_MODEL_NAME", "OFA-Sys/chinese-clip-vit-large-patch14"
        )
        self.embedding_dim = int(os.getenv("PROCESSING_EMBEDDING_DIM", "768"))

        self.device = get_device()
        logging.info("Using device: %s", self.device)

        self.embedder = ClipEmbedder(
            model_name=self.model_name,
            expected_dim=self.embedding_dim,
            device=self.device,
        )

        self.redis = redis.from_url(self.redis_url)

    def run_forever(self) -> None:
        logging.info("Text runner started. Waiting on queue=%s", self.queue)
        while True:
            task = self._pop()
            if not task:
                continue
            try:
                self._process(task)
            except Exception:
                logging.exception("Text task failed: %s", task)
                self._push_dlq(task)
                time.sleep(1)

    def _pop(self) -> TextTask | None:
        item = self.redis.brpop(self.queue, timeout=5)
        if item is None:
            return None
        _, payload = item
        try:
            return TextTask.from_raw(payload.decode("utf-8"))
        except Exception:
            logging.exception("Failed to parse text task: %s", payload)
            return None

    def _push_dlq(self, task: TextTask) -> None:
        try:
            self.redis.lpush(self.dlq, json.dumps(task.__dict__))
        except Exception:
            logging.exception("Failed to push task to DLQ")

    def _process(self, task: TextTask) -> None:
        embedding = self.embedder.embed_text(task.query)

        url = f"{self.api_base}/api/internal/text_queries/{task.request_id}"
        headers = {"x-internal-token": self.internal_token}
        payload = {"embedding": embedding, "model": task.model}
        post_json(url, payload, headers)
        logging.info("Text query %s processed", task.request_id)
