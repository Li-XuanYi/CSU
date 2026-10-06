#!/usr/bin/env python3
"""
LLaVA runner:
- Consume llava_tasks queue
- Generate responses for a single image conversation
- Callback internal API /api/internal/photo_conversations/{id}
"""

from __future__ import annotations

import json
import logging
import os
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional

import redis
from PIL import Image

from core import get_device, post_json, resolve_base_dir
from llava_engine import load_llava_engine

logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")


def get_env_bool(key: str, default: bool = False) -> bool:
    raw = os.getenv(key)
    if raw is None:
        return default
    return raw.strip() not in {"0", "false", "False", "no", "NO"}


def get_env_float(key: str, default: float) -> float:
    raw = os.getenv(key)
    if raw is None:
        return default
    try:
        return float(raw)
    except ValueError:
        return default


def get_env_int(key: str, default: int) -> int:
    raw = os.getenv(key)
    if raw is None:
        return default
    try:
        return int(raw)
    except ValueError:
        return default


@dataclass
class LlavaTask:
    conversation_id: int
    photo_id: int
    user_id: int
    path: str
    model: str
    history: List[Dict[str, Any]]
    params: Dict[str, Any]

    @classmethod
    def from_raw(cls, raw: str) -> "LlavaTask":
        data = json.loads(raw)
        return cls(
            conversation_id=int(data["conversation_id"]),
            photo_id=int(data["photo_id"]),
            user_id=int(data["user_id"]),
            path=data["path"],
            model=data.get("model") or "",
            history=data.get("history") or [],
            params=data.get("params") or {},
        )


class LlavaRunner:
    def __init__(self) -> None:
        self.redis_url = os.getenv("REDIS_URL", "redis://127.0.0.1:6379/0")
        self.queue = os.getenv("LLAVA_QUEUE_NAME", "llava_tasks")
        self.dlq = os.getenv("LLAVA_DLQ_NAME", f"{self.queue}_dlq")
        self.api_base = os.getenv("API_BASE_URL", "http://127.0.0.1:5800")
        self.internal_token = os.getenv("PROCESSING_INTERNAL_TOKEN", "change-me")
        self.model_name = os.getenv("LLAVA_MODEL_NAME", "liuhaotian/llava-v1.5-7b")
        self.conv_mode = os.getenv("LLAVA_CONV_MODE", "vicuna_v1")

        self.device = get_device()
        logging.info("Using device: %s", self.device)

        self.base_dir = resolve_base_dir()
        self.engine = load_llava_engine(self.model_name, self.device, self.conv_mode)

        self.redis = redis.from_url(self.redis_url)

    def run_forever(self) -> None:
        logging.info("LLaVA runner started. Waiting on queue=%s", self.queue)
        while True:
            task = self._pop()
            if not task:
                continue
            try:
                self._process(task)
            except Exception as exc:
                logging.exception("LLaVA task failed: %s", task)
                self._callback_error(task, str(exc))
                self._push_dlq(task)
                time.sleep(1)

    def _pop(self) -> Optional[LlavaTask]:
        item = self.redis.brpop(self.queue, timeout=5)
        if item is None:
            return None
        _, payload = item
        try:
            return LlavaTask.from_raw(payload.decode("utf-8"))
        except Exception:
            logging.exception("Failed to parse LLaVA task: %s", payload)
            return None

    def _push_dlq(self, task: LlavaTask) -> None:
        try:
            self.redis.lpush(self.dlq, json.dumps(task.__dict__))
        except Exception:
            logging.exception("Failed to push task to DLQ")

    def _process(self, task: LlavaTask) -> None:
        image_path = self._resolve_path(task.path)
        if not image_path.exists():
            raise FileNotFoundError(f"Image not found: {image_path}")

        if task.model and task.model != self.model_name:
            logging.warning(
                "Task model %s differs from runner model %s",
                task.model,
                self.model_name,
            )

        response = self._run_llava(task, image_path)
        url = f"{self.api_base}/api/internal/photo_conversations/{task.conversation_id}"
        headers = {"x-internal-token": self.internal_token}
        payload = {"response": response, "error": None}
        post_json(url, payload, headers)
        logging.info("LLaVA conversation %s processed", task.conversation_id)

    def _callback_error(self, task: LlavaTask, error: str) -> None:
        try:
            url = f"{self.api_base}/api/internal/photo_conversations/{task.conversation_id}"
            headers = {"x-internal-token": self.internal_token}
            payload = {"response": None, "error": error}
            post_json(url, payload, headers)
        except Exception:
            logging.exception("Failed to send LLaVA error callback")

    def _resolve_path(self, path: str) -> Path:
        raw = Path(path)
        if raw.is_absolute():
            return raw
        return self.base_dir.joinpath(path)

    def _run_llava(self, task: LlavaTask, image_path: Path) -> str:
        image = Image.open(image_path).convert("RGB")
        return self.engine.generate(image, task.history, task.params)


def main() -> None:
    runner = LlavaRunner()
    runner.run_forever()


if __name__ == "__main__":
    main()
