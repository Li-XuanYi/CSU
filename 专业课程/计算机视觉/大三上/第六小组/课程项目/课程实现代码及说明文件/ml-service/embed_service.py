#!/usr/bin/env python3
"""
轻量文本 embedding 服务：
- POST /embed_text {"text": "...", "model": "OFA-Sys/chinese-clip-vit-large-patch14"}
- 返回 {"embedding": [...]}

用途：后端可通过 PROCESSING_TEXT_EMBED_URL 调用本服务生成文本向量，再用 /api/search/text 搜索，无需前端/客户端生成 embedding。

运行：
```bash
cd ml
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
uvicorn embed_service:app --host 0.0.0.0 --port 8001
```

环境变量：
- CLIP_MODEL_NAME: 默认 OFA-Sys/chinese-clip-vit-large-patch14
- PROCESSING_EMBEDDING_DIM: 默认 768，用于警告尺寸不符
"""

import os
from typing import List, Optional

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

from core import get_clip_embedder


class EmbedRequest(BaseModel):
    text: str
    model: Optional[str] = None


class EmbedResponse(BaseModel):
    embedding: List[float]


def embed_text(text: str, model_name: str, expected_dim: int) -> List[float]:
    embedder = get_clip_embedder(model_name, expected_dim)
    return embedder.embed_text(text)


default_model = os.getenv(
    "CLIP_MODEL_NAME", "OFA-Sys/chinese-clip-vit-large-patch14"
)
expected_dim = int(os.getenv("PROCESSING_EMBEDDING_DIM", "768"))

app = FastAPI(title="Text Embedding Service", version="0.1.0")


@app.post("/embed_text", response_model=EmbedResponse)
def embed(req: EmbedRequest):
    text = req.text.strip()
    if not text:
        raise HTTPException(status_code=400, detail="text is required")

    model_name = req.model or default_model
    try:
        embedding = embed_text(text, model_name, expected_dim)
    except Exception as e:  # pragma: no cover
        raise HTTPException(status_code=500, detail=f"failed to embed text: {e}")

    return EmbedResponse(embedding=embedding)
