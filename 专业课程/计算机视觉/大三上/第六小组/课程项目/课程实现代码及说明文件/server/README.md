# Mindsight Gallery Server

基于 Rust + Salvo + Sea-ORM + PostgreSQL 的 Web 服务器项目，采用统一的 API 响应格式。

## 特性

✅ 统一的 API 响应格式 `{code, msg, data}`
✅ 完整的错误码体系
✅ 自动时间戳管理
✅ 数据库连接池
✅ Sea-ORM 异步 ORM
✅ 模块化项目结构
✅ 类型安全的请求处理

## 快速开始

### 1. 环境准备

```bash
# 安装 Rust (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 PostgreSQL (14+)
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt install postgresql
```

### 2. 配置数据库

```bash
# 创建数据库
createdb mindsight_gallery

# 或使用 psql
psql -U postgres
CREATE DATABASE mindsight_gallery;
\q
```

### 3. 配置环境变量

```bash
cp .env.example .env
# 编辑 .env 文件，修改数据库连接信息
DATABASE_URL=postgresql://你的用户名:你的密码@localhost:5432/mindsight_gallery
```

### 4. 运行数据库迁移

```bash
cargo run --bin migration
```

### 5. 启动服务器

```bash
cargo run
```

服务器将在 `http://0.0.0.0:5800` 启动。

### 6. 测试接口

```bash
# 健康检查
curl http://localhost:5800/health

# 返回示例
{
  "code": 0,
  "msg": null,
  "data": {
    "status": "ok"
  }
}
```

## 项目结构

```
├── src/
│   ├── main.rs           # 应用入口、路由配置
│   ├── lib.rs            # 库导出
│   ├── config.rs         # 配置管理
│   ├── db.rs             # 数据库连接池
│   ├── error.rs          # 错误处理、错误码定义
│   ├── response.rs       # 统一响应格式
│   ├── handlers/         # HTTP 请求处理器
│   │   ├── mod.rs
│   │   ├── health.rs     # 健康检查
│   │   └── users.rs      # 用户相关接口
│   ├── models/           # Sea-ORM 实体模型
│   │   ├── mod.rs
│   │   └── users.rs      # 用户模型（自动时间戳）
│   └── services/         # 业务逻辑层
│       └── mod.rs
├── migration/            # 数据库迁移
│   └── src/
│       ├── lib.rs
│       ├── main.rs
│       └── m20250125_000001_create_users_table.rs
├── .env.example          # 环境变量模板
├── API.md                # API 文档
└── Cargo.toml
```

## API 响应格式

所有接口统一使用以下格式：

```json
{
  "code": 0,           // 0:成功 其他:错误码
  "msg": null,         // 成功:null 失败:错误信息
  "data": { ... }      // 成功:数据 失败:null
}
```

详见 [API.md](./API.md)

## 数据库最佳实践

### 自动时间戳管理

```rust
// models/users.rs 中实现了 ActiveModelBehavior
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        if insert {
            self.created_at = Set(now);
            self.updated_at = Set(now);
        } else {
            self.updated_at = Set(now);
        }
        Ok(self)
    }
}
```

### 数据库迁移

```bash
# 运行所有待执行的迁移
cargo run --bin migration

# 创建新迁移（在 migration/src/ 目录手动创建）
# 文件命名格式: m{YYYYMMDD}_{序号}_{描述}.rs
```

## 开发指南

### 添加新接口

1. 在 `src/handlers/` 创建处理器
2. 在 `src/main.rs` 注册路由
3. 使用统一响应格式 `ApiResponse`

示例：

```rust
use salvo::prelude::*;
use crate::response::ApiResponse;

#[handler]
pub async fn example() -> Json<ApiResponse<YourData>> {
    Json(ApiResponse::success(your_data))
}
```

### 访问数据库

```rust
#[handler]
pub async fn handler(depot: &mut Depot) -> Result<Json<ApiResponse<Data>>, AppError> {
    let db = depot.obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("DB not found".into()))?;

    let results = YourEntity::find().all(db.as_ref()).await?;
    Ok(Json(ApiResponse::success(results)))
}
```

### 错误处理

```rust
// 返回错误会自动转换为统一格式
return Err(AppError::NotFound("User not found".to_string()));

// 转换为：
// {
//   "code": 1003,
//   "msg": "Not found: User not found",
//   "data": null
// }
```

## 技术栈

- **Web 框架**: Salvo 0.80 - 高性能异步 Web 框架
- **ORM**: Sea-ORM 1.1 - 异步 ORM，支持自动时间戳
- **数据库**: PostgreSQL - 强大的开源关系型数据库
- **异步运行时**: Tokio - Rust 标准异步运行时
- **日志**: tracing - 结构化日志
- **时间处理**: chrono - 日期时间库

## 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| SERVER_HOST | 服务器监听地址 | 0.0.0.0 |
| SERVER_PORT | 服务器端口 | 5800 |
| DATABASE_URL | 数据库连接字符串 | *必填* |
| DB_MAX_CONNECTIONS | 最大连接数 | 10 |
| DB_MIN_CONNECTIONS | 最小连接数 | 2 |

## 常用命令

```bash
# 开发环境运行
cargo run

# 检查代码
cargo check

# 运行测试
cargo test

# 生产构建
cargo build --release

# 运行迁移
cargo run --bin migration

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## License

MIT
