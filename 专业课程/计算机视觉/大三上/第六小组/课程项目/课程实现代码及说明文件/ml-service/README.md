# ML Runners

本目录负责与服务端的 ML 处理流程对接：图片上传后进入 Redis 队列，runner 生成向量/结果并回写内部接口；文本检索可走同步 embedding 服务或异步队列。

## 环境准备

```bash
cd ml
python -m venv .venv
source .venv/bin/activate  # Windows 使用 .venv\Scripts\activate
pip install -r requirements.txt
```

首次运行会从 HuggingFace 下载模型（如 `OFA-Sys/chinese-clip-vit-large-patch14`），离线可预先设置 `HF_HOME` 并缓存模型。
Caption 依赖 BLIP 模型接口，要求 `transformers>=4.40`（已在 `requirements.txt` 中声明）。

## 统一入口（Master Runner）

```bash
cd ml
python master_runner.py clip          # 图片任务队列
python master_runner.py text          # 文本 embedding 异步队列
python master_runner.py embed-service # 文本 embedding 服务
python master_runner.py all           # clip + text 同进程运行
python master_runner.py text-search "a cat on the beach" --limit 10
python master_runner.py people-album
python master_runner.py llava
```

说明：`clip_runner.py`/`text_runner.py`/`text_search.py`/`people_album.py`/`embed_service.py` 仅保留业务逻辑与数据结构，入口统一由 `master_runner.py` 管理。

## 服务端实际流程对齐

- 图片上传后，服务端会写入 Redis `photo_tasks`（`config.redis.queue_name`）：
  - `tasks` 通常包含 `thumbnail/clip/face/ocr`
  - runner 只处理 `clip/face/ocr`，缩略图由服务端自行生成
- Runner 回写接口：
  - `POST /api/internal/photos/{photo_id}/ml_result`：保存 `image/caption` embedding + `ml_result`
  - `POST /api/internal/photos/{photo_id}/faces`：保存人脸 embedding 列表
- 文本搜图 `/api/search/text`：
  - 如果请求内带 `embedding`，直接同步搜索
  - 若配置了 `PROCESSING_TEXT_EMBED_URL`，服务端会调用文本 embedding 服务
  - 否则会写入 `text_tasks`（`config.processing.text_embed_queue`）并由 `text_runner` 回调
- 异步回调：`POST /api/internal/text_queries/{id}` 写入 embedding 并触发检索

## 关键环境变量

与服务端配置保持一致（建议放 `.env`）：

```
# 通用
API_BASE_URL=http://127.0.0.1:5800
PROCESSING_INTERNAL_TOKEN=与服务端一致的 token

# 图片队列
REDIS_URL=redis://127.0.0.1:6379/0
REDIS_QUEUE_NAME=photo_tasks
REDIS_DLQ_NAME=photo_tasks_dlq

# 文本队列
TEXT_EMBED_QUEUE_NAME=text_tasks
TEXT_EMBED_DLQ_NAME=text_tasks_dlq

# 文本搜索与相册
ACCESS_TOKEN=你的 JWT

# 模型/维度（必须匹配服务端）
CLIP_MODEL_NAME=OFA-Sys/chinese-clip-vit-large-patch14
PROCESSING_EMBEDDING_DIM=768
PROCESSING_DEFAULT_MODEL=chinese-clip-vit-large-patch14
PROCESSING_FACE_EMBEDDING_DIM=512
PROCESSING_DEFAULT_FACE_MODEL=arcface-buffalo_l

# 可选：多 CLIP 模型
CLIP_MODELS=zhclip=OFA-Sys/chinese-clip-vit-large-patch14

# 文本 embedding 服务（供服务端调用）
PROCESSING_TEXT_EMBED_URL=http://127.0.0.1:8001/embed_text

# Caption（默认关闭）
CAPTION_ENABLED=0
CAPTION_MODEL_NAME=Salesforce/blip-image-captioning-base
CAPTION_MAX_LENGTH=64

# OCR（可关闭）
OCR_ENABLED=1
OCR_LANGS=ch_sim,en
OCR_MIN_CONF=0.3
OCR_MAX_CHARS=2000

# 人脸模型
FACE_MODEL_NAME=buffalo_l

# 文本融合检索权重（服务端读取）
TEXT_FUSION_IMAGE_WEIGHT=1.0
TEXT_FUSION_CAPTION_WEIGHT=0.0
TEXT_FUSION_OCR_WEIGHT=0.2

# 上传路径前缀（相对路径时用于拼接）
PROJECT_ROOT=/home/xxx/mindsight-gallery-server
# 或 UPLOAD_BASE_DIR=/home/xxx/mindsight-gallery-server

# LLaVA（图片对话）
LLAVA_QUEUE_NAME=llava_tasks
LLAVA_MODEL_NAME=liuhaotian/llava-v1.5-7b
LLAVA_CONV_MODE=vicuna_v1
LLAVA_LOAD_4BIT=1
LLAVA_MEMVR_START_LAYER=5
LLAVA_MEMVR_END_LAYER=16
LLAVA_MEMVR_ENTROPY_THRESHOLD=0.75
LLAVA_MEMVR_RETRACING_RATIO=0.1
LLAVA_TEMPERATURE=0
LLAVA_TOP_P=0.9
LLAVA_NUM_BEAMS=1
LLAVA_MAX_NEW_TOKENS=512
```

## 文搜图（文本->图像）

`text-search` 会在本地生成文本 embedding，并调用 `/api/search/vector` 返回相似图片。  
若希望服务端自动编码文本，可直接调用 `/api/search/text` 并让服务端走 `PROCESSING_TEXT_EMBED_URL` 或 `text_tasks`。

需要环境变量：

```
API_BASE_URL=http://127.0.0.1:5800
ACCESS_TOKEN=你的 JWT
CLIP_MODEL_NAME=OFA-Sys/chinese-clip-vit-large-patch14
PROCESSING_EMBEDDING_DIM=768
PROCESSING_DEFAULT_MODEL=chinese-clip-vit-large-patch14
```

运行示例：

```bash
python master_runner.py text-search "a cat on the beach" --limit 10
```

## 自动生成人物相册

`people-album` 会读取已上传图片，做人脸聚类并创建相册。

需要环境变量：

```
API_BASE_URL=http://127.0.0.1:5800
ACCESS_TOKEN=你的 JWT
PROJECT_ROOT=/home/xxx/mindsight-gallery-server
FACE_MODEL_NAME=buffalo_l
FACE_DISTANCE_THRESHOLD=0.42
MIN_FACES_PER_PERSON=2
MAX_PHOTOS=0
```

运行示例：

```bash
python master_runner.py people-album
```

## 图片对话（LLaVA）

`llava` runner 从 Redis `llava_tasks` 队列消费任务，生成图片对话回复并回调服务端。
需要提前安装 LLaVA 依赖（参考 `ml/llava_test`）。

运行示例：

```bash
python master_runner.py llava
```

## 任务格式（photo_tasks）

```json
{
  "photo_id": 1,
  "user_id": 1,
  "path": "./uploads/original/xxx.jpg",
  "tasks": ["thumbnail", "clip", "face", "ocr"]
}
```

## 说明

- embedding 写入时会校验维度；需与 `PROCESSING_EMBEDDING_DIM` / `PROCESSING_FACE_EMBEDDING_DIM` 一致。
- 任务里的图片路径若为相对路径，runner 会用仓库根目录（`ml` 上一级）拼接；可通过 `PROJECT_ROOT` 或 `UPLOAD_BASE_DIR` 覆盖。
- OCR 结果会写入 `ml_result.ocr`，Caption 会以 `modality=caption` 写入 embedding，供服务端融合检索使用。
