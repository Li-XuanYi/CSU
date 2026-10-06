# 代码说明文档（后端 / 前端 / 机器学习）

本项目由 Rust 后端、Vite + React 前端、Python 机器学习 Runner 组成。后端提供 API 与内部回调接口；前端提供 UI；ML Runner 负责向量化、人脸识别、OCR、对话等异步处理。

## 目录结构速览

```
.
├── src/                 # Rust 后端主工程
├── migration/           # Sea-ORM 数据库迁移
├── web/                 # 前端（Vite + React）
└── ml/                  # 机器学习 Runner（Python）
```

## 运行环境要求

### 后端（Rust）
- Rust 工具链需支持 edition 2024（建议 Rust 1.85+）
- PostgreSQL 14+（必需）
- Redis（ML 队列/异步任务用；若只跑基础 API 可选）

### 前端（Web）
- Node.js 18+（Vite 6 要求）
- npm（或 pnpm / yarn 兼容）

### 机器学习（ML）
- Python 3.10+（建议）
- 可选 GPU（CLIP / LLaVA / InsightFace 速度显著提升）
- 首次运行会从 HuggingFace 下载模型，可设置 `HF_HOME` 预缓存

## 依赖库要求（按模块）

### 后端关键依赖（`Cargo.toml`）
- Web 框架：`salvo`
- ORM/迁移：`sea-orm`, `sea-orm-migration`
- 运行时：`tokio`
- 配置/序列化：`dotenvy`, `serde`, `serde_json`
- Redis/HTTP：`redis`, `reqwest`
- 安全：`bcrypt`, `jsonwebtoken`
- 图片/EXIF：`image`, `kamadak-exif`

### 前端关键依赖（`web/package.json`）
- `react`, `react-dom`
- `react-router-dom`
- `lucide-react`
- 开发工具：`vite`, `typescript`

### ML 关键依赖（`ml/requirements*.txt`）
- 视觉/语言模型：`torch`, `transformers`, `timm`
- 人脸：`insightface`, `opencv-python-headless`
- OCR：`easyocr`
- 服务/队列：`fastapi`, `uvicorn`, `redis`, `requests`
- LLaVA 单独环境：`requirements-llava.txt`（固定 `transformers==4.37.2` 等版本）

## 配置与环境变量

### 后端（`.env`）
参考 `.env.example`，并注意以下配置：

- `SERVER_HOST`, `SERVER_PORT`：服务监听地址与端口（默认 `0.0.0.0:5800`）
- `DATABASE_URL`：PostgreSQL 连接串（必填）
- `DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`：连接池
- `UPLOAD_DIR`, `THUMBNAIL_DIR`：存储路径（默认 `./uploads/original` 与 `./uploads/thumbnails`）
- `PROCESSING_INTERNAL_TOKEN`：ML 回调鉴权 token（需与 ML 一致）
- `PROCESSING_EMBEDDING_DIM` / `PROCESSING_FACE_EMBEDDING_DIM`：向量维度
- `REDIS_URL`, `REDIS_QUEUE_NAME`：任务队列（默认 `photo_tasks`）

### 前端（`web/.env.local`）
- `GEMINI_API_KEY`：前端页面使用 Gemini 时需要

### ML（建议放 `ml/.env`）
最小可用配置：

```
API_BASE_URL=http://127.0.0.1:5800
PROCESSING_INTERNAL_TOKEN=与后端一致
REDIS_URL=redis://127.0.0.1:6379/0
REDIS_QUEUE_NAME=photo_tasks
TEXT_EMBED_QUEUE_NAME=text_tasks
LLAVA_QUEUE_NAME=llava_tasks
CLIP_MODEL_NAME=OFA-Sys/chinese-clip-vit-large-patch14
PROCESSING_EMBEDDING_DIM=768
PROCESSING_DEFAULT_MODEL=chinese-clip-vit-large-patch14
PROCESSING_FACE_EMBEDDING_DIM=512
PROCESSING_DEFAULT_FACE_MODEL=arcface-buffalo_l
```

完整说明见 `ml/README.md`。

## 启动与运行（建议顺序）

### 1) 数据库迁移
```bash
cargo run --bin migration
```

### 2) 启动后端
```bash
cargo run
```
默认监听 `http://0.0.0.0:5800`。

### 3) 启动 ML Runner（可选）
```bash
cd ml
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python master_runner.py all
```
如需 LLaVA，建议单独创建环境并安装 `ml/requirements-llava.txt`。

### 4) 启动前端
```bash
cd web
npm install
npm run dev
```

## 其它注意事项

- 迁移与后端是同一 Rust workspace，迁移需先配置好 `DATABASE_URL`。
- 上传目录需可写；若使用相对路径，ML Runner 会按仓库根目录拼接。
- ML 与后端的向量维度、模型名必须一致，否则会触发校验错误。
- LLaVA 依赖版本敏感，建议与主 ML 环境分开安装。
