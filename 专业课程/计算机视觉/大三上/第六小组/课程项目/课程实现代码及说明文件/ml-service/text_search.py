#!/usr/bin/env python3
"""
文搜图 CLI：
- 将输入文本编码为 CLIP 文本向量
- 调用服务端 /api/search/vector 接口返回相似图片

依赖环境变量（可放在 .env）：
- API_BASE_URL: 服务地址，默认 http://127.0.0.1:5800
- ACCESS_TOKEN: 用户的 Bearer Token（必需，来自登录接口）
- CLIP_MODEL_NAME: 文本编码模型，默认 OFA-Sys/chinese-clip-vit-large-patch14
- PROCESSING_EMBEDDING_DIM: 维度，默认 768
- PROCESSING_DEFAULT_MODEL: 写入的模型标识，默认 chinese-clip-vit-large-patch14
"""

import logging
import os
from typing import List

import requests

from core import ClipEmbedder, get_device


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")


class TextSearch:
    def __init__(self) -> None:
        self.api_base = os.getenv("API_BASE_URL", "http://127.0.0.1:5800")
        self.access_token = os.getenv("ACCESS_TOKEN")
        if not self.access_token:
            raise SystemExit("环境变量 ACCESS_TOKEN 为空，请先登录并填入 JWT")

        self.model_name = os.getenv(
            "CLIP_MODEL_NAME", "OFA-Sys/chinese-clip-vit-large-patch14"
        )
        self.embedding_dim = int(os.getenv("PROCESSING_EMBEDDING_DIM", "768"))
        self.default_model_label = os.getenv(
            "PROCESSING_DEFAULT_MODEL", "chinese-clip-vit-large-patch14"
        )

        self.device = get_device()
        logging.info("Using device: %s", self.device)

        self.embedder = ClipEmbedder(
            model_name=self.model_name,
            expected_dim=self.embedding_dim,
            device=self.device,
        )

    def embed_text(self, query: str) -> List[float]:
        return self.embedder.embed_text(query)

    def search(self, query: str, limit: int = 20) -> None:
        embedding = self.embed_text(query)
        payload = {
            "embedding": embedding,
            "model": self.default_model_label,
            "modality": "text",
            "limit": limit,
        }

        url = f"{self.api_base}/api/search/vector"
        headers = {"Authorization": f"Bearer {self.access_token}"}
        resp = requests.post(url, json=payload, headers=headers, timeout=30)
        if resp.status_code >= 300:
            raise SystemExit(f"搜索失败: {resp.status_code} {resp.text}")

        data = resp.json()
        # 接口直接返回 Vec<SimilarPhotoResult>，或统一响应包裹
        results = data.get("data") if isinstance(data, dict) and "data" in data else data
        if not results:
            print("没有找到匹配的图片")
            return

        print(f"\n查询: {query}\n结果(按相似度升序)：")
        for idx, item in enumerate(results, 1):
            photo = item.get("photo", {})
            distance = item.get("distance")
            print(
                f"{idx}. photo_id={photo.get('id')} url={photo.get('url')} distance={distance}"
            )
