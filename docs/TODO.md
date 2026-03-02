# 项目待实现功能清单

本文档记录了项目中已规划但尚未实现或未完整实现的功能。

**最后更新**: 2026-03-02
**项目**: rust-server-demo

---

## 🔴 高优先级

### 1. 速率限制（Rate Limiting）

**状态**: ⚠️ 基础设施已就绪，但功能未实现

**现有基础**:
- ✅ `RateLimitExceeded` 错误类型已定义 (`src/core/error.rs:84`)
- ✅ 缓存键生成函数已实现 (`src/cache/keys.rs:14,58-59`)
- ✅ Redis incr 操作已支持 (`src/cache/client.rs:97-101`)

**缺失部分**:
- ❌ 速率限制中间件
- ❌ 限流配置（每分钟请求数、突发限制）
- ❌ 不同端点的限流策略

**实现建议**:
```rust
// src/middleware/rate_limit.rs
pub async fn rate_limit_middleware(
    State(redis): State<RedisClient>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let identifier = extract_ip_or_user(&req);
    let key = CacheKeys::rate_limit(&identifier);

    let count = redis.incr(&key).await?;
    if count == 1 {
        redis.expire(&key, 60).await?; // 1 分钟窗口
    }

    if count > 100 { // 每分钟 100 次请求
        return Err(AppError::RateLimitExceeded);
    }

    Ok(next.run(req).await)
}
```

**涉及文件**:
- 新建 `src/middleware/rate_limit.rs`
- 修改 `src/routes/api.rs` - 添加中间件
- 修改 `src/config/base.rs` - 添加限流配置

---

### 2. WebSocket 房间广播功能

**状态**: ⚠️ 基础架构完整，但核心逻辑未实现

**现有基础**:
- ✅ WebSocket 连接管理器 (`src/websocket/connection.rs`)
- ✅ 广播器 (`src/websocket/broadcast.rs`)
- ✅ 消息模型 (`src/models/message.rs`)
- ✅ Redis 房间成员管理 (`src/cache/keys.rs:22-23`)

**缺失部分**:
- ❌ 实际的房间加入/离开逻辑
- ❌ 房间消息广播（当前仅 echo）
- ❌ 房间成员列表查询
- ❌ 房间消息持久化集成

**实现建议**:
```rust
// src/handlers/ws.rs - 处理 Chat 消息
ClientMessage::Chat { room_id, content } => {
    // 1. 保存消息到数据库
    let message = message_service.create_message(
        user_id, &room_id, &content
    ).await?;

    // 2. 广播给房间所有成员
    broadcaster.broadcast_to_room(&room_id, ServerMessage::Message {
        id: message.id,
        sender_id: user_id,
        content: message.content,
        timestamp: message.created_at,
    }).await?;
}
```

**涉及文件**:
- 修改 `src/handlers/ws.rs:153-160` - 实现 Chat 分支
- 修改 `src/websocket/connection.rs` - 添加房间操作方法
- 修改 `src/websocket/broadcast.rs` - 添加 `broadcast_to_room`

---

## 🟡 中优先级

### 3. 会话管理 HTTP API

**状态**: ⚠️ 服务层完整实现，但 API 端点缺失

**现有实现**:
- ✅ `SessionService` 完整实现 (`src/services/session_service.rs`)
- ✅ `SessionRepository` 完整实现 (`src/database/repositories/session.rs`)
- ✅ 数据库表和迁移 (`migrations/`)

**缺失部分**:
- ❌ HTTP 处理器
- ❌ API 路由注册
- ❌ OpenAPI 文档

**需要的 API 端点**:
```
GET    /api/sessions           - 列出当前用户的所有活跃会话
GET    /api/sessions/:id       - 获取指定会话详情
DELETE /api/sessions/:id       - 销毁指定会话
DELETE /api/sessions/other     - 销毁除当前会话外的所有会话
GET    /api/sessions/active    - 获取活跃会话统计
```

**实现建议**:
```rust
// 新建 src/handlers/session.rs
pub async fn list_sessions(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> AppResult<Json<Vec<SessionDto>>> {
    let sessions = session_service
        .get_user_sessions(user.id)
        .await?;
    Ok(Json(sessions))
}
```

**涉及文件**:
- 新建 `src/handlers/session.rs`
- 修改 `src/routes/api.rs` - 添加会话路由
- 修改 `src/state.rs` - 暴露 `SessionService`

---

### 4. 消息 HTTP API

**状态**: ⚠️ 服务层完整，但 HTTP 接口缺失

**现有实现**:
- ✅ `MessageService` 完整实现 (`src/services/message_service.rs`)
- ✅ 消息模型和验证 (`src/models/message.rs`)

**缺失部分**:
- ❌ HTTP 处理器
- ❌ REST API 端点

**需要的 API 端点**:
```
POST   /api/messages           - 创建消息
GET    /api/messages           - 列出消息（分页，按房间/用户过滤）
GET    /api/messages/:id       - 获取单条消息
PUT    /api/messages/:id       - 编辑消息
DELETE /api/messages/:id       - 删除消息
GET    /api/rooms/:id/messages - 获取房间消息历史
```

**涉及文件**:
- 新建 `src/handlers/message.rs`
- 修改 `src/routes/api.rs` - 添加消息路由
- 修改 `src/state.rs` - 暴露 `MessageService`

---

### 5. 房间管理系统

**状态**: ❌ 完全未实现

**需要实现**:
- ❌ 房间数据库表定义
- ❌ 房间模型和 DTO
- ❌ RoomRepository
- ❌ RoomService
- ❌ 房间权限管理
- ❌ HTTP API 处理器

**数据库表设计**:
```sql
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id),
    is_private BOOLEAN NOT NULL DEFAULT false,
    max_members INT DEFAULT 100,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE room_members (
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- owner, admin, member
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (room_id, user_id)
);
```

**需要的 API 端点**:
```
POST   /api/rooms              - 创建房间
GET    /api/rooms              - 列出房间
GET    /api/rooms/:id          - 获取房间详情
PUT    /api/rooms/:id          - 更新房间信息
DELETE /api/rooms/:id          - 删除房间
POST   /api/rooms/:id/join     - 加入房间
POST   /api/rooms/:id/leave    - 离开房间
GET    /api/rooms/:id/members  - 获取房间成员列表
PUT    /api/rooms/:id/members/:user_id - 更新成员角色
```

**涉及文件**:
- 新建 `migrations/YYYYMMDD_create_rooms.sql`
- 新建 `src/models/room.rs`
- 新建 `src/database/repositories/room.rs`
- 新建 `src/services/room_service.rs`
- 新建 `src/handlers/room.rs`
- 修改 `src/routes/api.rs`

---

### 6. 电子邮件验证流程

**状态**: ⚠️ 基础字段存在，但验证流程未实现

**现有基础**:
- ✅ `is_verified` 字段在用户模型中 (`src/models/user.rs:19`)
- ✅ `verify_email()` 方法在服务层 (`src/services/user_service.rs:271-290`)

**缺失部分**:
- ❌ 邮件发送服务（SMTP 集成）
- ❌ 验证令牌生成和存储
- ❌ 验证链接生成
- ❌ 验证 API 端点
- ❌ 重新发送验证邮件功能

**实现建议**:

1. **添加邮件服务**:
```toml
# Cargo.toml
lettre = "0.11"
```

2. **创建验证令牌表**:
```sql
CREATE TABLE email_verification_tokens (
    token VARCHAR(64) PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

3. **API 端点**:
```
POST /api/auth/verify-email           - 验证邮箱（带 token）
POST /api/auth/resend-verification    - 重新发送验证邮件
```

**涉及文件**:
- 新建 `src/services/email_service.rs`
- 新建 `src/models/verification.rs`
- 新建 `migrations/YYYYMMDD_create_verification_tokens.sql`
- 修改 `src/handlers/auth.rs` - 添加验证端点
- 修改 `src/config/base.rs` - 添加 SMTP 配置

---

### 7. 请求限制中间件配置应用

**状态**: ⚠️ 配置已定义但未实际应用

**现有配置**:
```rust
// src/config/base.rs
pub max_body_size: u64,      // 已定义但未应用
pub request_timeout: u64,     // 已定义但未应用
```

**需要修改**:
```rust
// src/main.rs
let app = Router::new()
    .merge(api_routes)
    .layer(
        tower::ServiceBuilder::new()
            .layer(RequestBodyLimitLayer::new(
                config.max_body_size * 1024 * 1024 // MB to bytes
            ))
            .layer(TimeoutLayer::new(
                Duration::from_secs(config.request_timeout)
            ))
    );
```

**涉及文件**:
- 修改 `src/main.rs:200-250` - 应用中间件层

---

## 🟢 低优先级

### 8. Repository 单元测试

**状态**: ⚠️ 仅有占位符测试

**现状**:
- `SessionRepository` 仅有占位符测试 (`src/database/repositories/session.rs:195-201`)
- `UserRepository` 没有测试块
- 需要完整的单元测试覆盖

**实现建议**:
使用 SQLx 的测试支持或 Docker testcontainers：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_test_db() -> PgPool {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(pool);
        // ... 测试逻辑
    }
}
```

**涉及文件**:
- 修改 `src/database/repositories/user.rs` - 添加测试
- 修改 `src/database/repositories/session.rs` - 完善测试
- 新建 `.env.test` - 测试数据库配置

---

### 9. 活动日志自动记录中间件

**状态**: ⚠️ 服务层完整，但未自动集成

**现有实现**:
- ✅ `ActivityService` 完整实现
- ✅ 活动日志表和迁移

**缺失部分**:
- ❌ 自动记录 HTTP 请求的中间件
- ❌ 关键操作的活动日志集成

**实现建议**:
```rust
// src/middleware/activity_logger.rs
pub async fn activity_logger_middleware(
    State(activity_service): State<ActivityService>,
    AuthenticatedUser(user): AuthenticatedUser,
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let response = next.run(req).await;

    // 仅记录重要操作（POST, PUT, DELETE）
    if matches!(method, Method::POST | Method::PUT | Method::DELETE) {
        let _ = activity_service.create_activity(
            user.id,
            &format!("{} {}", method, path),
            "http_request",
            None,
        ).await;
    }

    response
}
```

**涉及文件**:
- 新建 `src/middleware/activity_logger.rs`
- 修改 `src/routes/api.rs` - 应用中间件

---

### 10. Kafka 消费者实现

**状态**: ⚠️ 仅有生产者，无消费者

**现有实现**:
- ✅ Kafka 生产者 (`src/kafka/producer.rs`)
- ✅ 用户和活动事件发布

**缺失部分**:
- ❌ Kafka 消费者
- ❌ 消息处理逻辑
- ❌ 错误恢复和重试
- ❌ 消费者组管理

**实现建议**:
```rust
// src/kafka/consumer.rs
pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub async fn consume_user_events(&self) -> Result<(), KafkaError> {
        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        // 处理用户事件
                        self.handle_user_event(payload).await?;
                    }
                    self.consumer.commit_message(&message, CommitMode::Async)?;
                }
                Err(e) => {
                    tracing::error!("Kafka consumer error: {:?}", e);
                }
            }
        }
    }
}
```

**涉及文件**:
- 新建 `src/kafka/consumer.rs` (feature-gated)
- 修改 `src/main.rs` - 启动消费者任务

---

### 11. 缓存预热集成

**状态**: ✅ 功能已实现，但未集成到启动流程

**现有实现**:
- ✅ `warm_up_cache()` 方法 (`src/services/user_service.rs:205-236`)
- ✅ `warm_up_recent_active_users()` 方法

**建议集成**:
```rust
// src/main.rs - 在服务启动后
tracing::info!("Warming up cache...");
let warmup_count = user_service
    .warm_up_recent_active_users(100)
    .await
    .unwrap_or(0);
tracing::info!("Cached {} hot users", warmup_count);
```

**涉及文件**:
- 修改 `src/main.rs` - 添加缓存预热

---

### 12. 数据库 Seeder（测试数据生成）

**状态**: ❌ 完全缺失

**用途**:
- 生成演示数据
- 填充开发环境
- E2E 测试数据准备

**实现建议**:
```rust
// src/database/seeder.rs
pub async fn seed_users(pool: &PgPool, count: usize) -> Result<()> {
    for i in 0..count {
        let username = format!("user{}", i);
        let email = format!("user{}@example.com", i);
        // ... 插入用户
    }
    Ok(())
}

pub async fn seed_all(pool: &PgPool) -> Result<()> {
    seed_users(pool, 10).await?;
    seed_rooms(pool, 5).await?;
    seed_messages(pool, 100).await?;
    Ok(())
}
```

**涉及文件**:
- 新建 `src/database/seeder.rs`
- 新建命令行参数 `--seed` 在 `src/main.rs`

---

## 📊 统计总结

| 优先级 | 功能数量 | 已完成 | 进行中 | 未开始 |
|--------|----------|--------|--------|--------|
| 🔴 高 | 2 | 0 | 2 | 0 |
| 🟡 中 | 7 | 0 | 5 | 2 |
| 🟢 低 | 5 | 1 | 1 | 3 |
| **总计** | **14** | **1** | **8** | **5** |

---

## 🎯 建议实现顺序

### 第一阶段（核心功能完善）
1. ✅ Redis 缓存改进（已完成）
2. 速率限制中间件
3. WebSocket 房间广播

### 第二阶段（API 完善）
4. 会话管理 HTTP API
5. 消息 HTTP API
6. 房间管理系统

### 第三阶段（增强功能）
7. 电子邮件验证流程
8. 请求限制中间件应用
9. 活动日志自动记录

### 第四阶段（测试和优化）
10. Repository 单元测试
11. 缓存预热集成
12. 数据库 Seeder

### 第五阶段（扩展功能）
13. Kafka 消费者
14. 其他优化和增强

---

## 📝 注意事项

1. **测试覆盖**: 每个新功能都应编写相应的单元测试和集成测试
2. **文档更新**: 实现功能后及时更新 CLAUDE.md 和 API 文档
3. **向后兼容**: 添加新功能时注意保持 API 的向后兼容性
4. **性能考虑**: 新增中间件和功能可能影响性能，需要进行基准测试
5. **安全审查**: 特别是速率限制、会话管理等安全相关功能需要仔细审查

---

**维护者**: 请在完成功能后及时更新此文档，标记完成状态并移除相应项。
