# Kafka 活动日志系统 - 快速开始

## 概述

本演示展示了如何使用 Kafka 构建一个实时活动日志系统，用于跟踪用户操作和系统事件。

## 功能特性

✅ **实时事件追踪** - 记录所有用户操作（登录、注册、消息等）
✅ **Kafka 集成** - 事件发布到 Kafka topic 进行异步处理
✅ **Redis 缓存** - 最近活动缓存提升查询性能
✅ **前端展示** - 实时活动流界面，支持自动刷新
✅ **分页查询** - 支持历史活动记录的分页浏览
✅ **元数据支持** - 灵活的事件元数据存储

## 快速开始

### 1. 启动基础设施

```bash
# 启动 Docker Compose 服务（PostgreSQL, Redis, Kafka）
docker-compose up -d

# 验证服务状态
docker-compose ps
```

### 2. 启动后端服务

```bash
# 设置环境变量
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_server"
export REDIS_URL="redis://localhost:6379"
export KAFKA_BROKERS="localhost:9092"
export JWT_SECRET="your-very-secret-jwt-key-at-least-32-characters-long"

# 编译并运行（启用 Kafka 功能）
cargo run --features kafka
```

后端将在 `http://localhost:8080` 启动

### 3. 启动前端服务

```bash
cd frontend
npm install
npm run dev
```

前端将在 `http://localhost:3000` 启动

### 4. 访问活动流界面

1. 在浏览器打开 `http://localhost:3000`
2. 注册/登录一个账户
3. 访问 `http://localhost:3000/activities` 查看活动流
4. 执行一些操作（如登录、查看用户列表等）
5. 活动流会自动每5秒刷新显示最新活动

## API 端点

### 获取最近活动（缓存）

```bash
GET /api/activities/recent
Authorization: Bearer <token>
```

响应示例：
```json
{
  "status": 200,
  "statusText": "OK",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "123e4567-e89b-12d3-a456-426614174000",
      "username": "john_doe",
      "event_type": "user_logged_in",
      "event_type_category": "user",
      "description": "User logged in successfully",
      "metadata": {
        "login_method": "password"
      },
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

### 获取所有活动（分页）

```bash
GET /api/activities?page=1&per_page=20
Authorization: Bearer <token>
```

### 获取特定用户的活动

```bash
GET /api/activities/user/{user_id}?page=1&per_page=20
Authorization: Bearer <token>
```

### 创建活动日志

```bash
POST /api/activities
Authorization: Bearer <token>
Content-Type: application/json

{
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "event_type": "custom_event",
  "description": "A custom event occurred",
  "metadata": {
    "key": "value"
  },
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0..."
}
```

## 支持的事件类型

### 用户事件 (user_*)
- `user_registered` - 用户注册
- `user_logged_in` - 用户登录
- `user_logged_out` - 用户登出
- `user_updated` - 用户信息更新
- `user_password_changed` - 密码修改

### 消息事件 (message_*)
- `message_sent` - 发送消息
- `message_edited` - 编辑消息
- `message_deleted` - 删除消息

### 房间事件 (room_*)
- `room_joined` - 加入房间
- `room_left` - 离开房间
- `room_created` - 创建房间

### 系统事件 (system_*)
- `system_alert` - 系统告警
- `system_notification` - 系统通知

### 错误事件 (error_*)
- `error_occurred` - 错误发生

## 技术架构

### 后端技术栈

- **Rust** - 高性能系统编程语言
- **Axum** - Web 框架
- **SQLx** - 数据库 ORM
- **rdkafka** - Kafka 客户端
- **Redis** - 缓存

### 前端技术栈

- **SvelteKit** - 全栈框架
- **Svelte 5** - UI 框架（使用 Runes）
- **Tailwind CSS v4** - 样式
- **shadcn-svelte** - UI 组件

### 数据流

```
用户操作 → Handler → ActivityService → PostgreSQL
                              ↓
                         Kafka Producer → Kafka Topic
                              ↓
                         Redis Cache (5 min)
```

## 性能优化

1. **Redis 缓存**
   - 最近50条活动缓存
   - 5分钟 TTL
   - 减少90%数据库查询

2. **Kafka 异步处理**
   - 不阻塞主业务流程
   - 批量发送优化
   - 自动重试机制

3. **数据库索引**
   - user_id 索引
   - event_type 索引
   - created_at 索引

## 故障排查

### Kafka 连接失败

```bash
# 检查 Kafka 是否运行
docker ps | grep kafka

# 查看 Kafka 日志
docker logs kafka

# 测试 Kafka 连接
nc -zv localhost 9092
```

### 活动未记录

1. 检查后端日志：`RUST_LOG=debug cargo run --features kafka`
2. 验证 Kafka topic 是否存在：`kafka-topics.sh --list --bootstrap-server localhost:9092`
3. 查看 API 响应：使用 Swagger UI `http://localhost:8080/swagger-ui`

### 前端不更新

1. 打开浏览器开发者工具查看网络请求
2. 验证认证 token 是否有效
3. 检查 `/api/activities/recent` 端点响应

## 扩展建议

### 添加 Kafka Consumer

创建后台服务消费活动事件：

```rust
#[cfg(feature = "kafka")]
use tokio::time::{interval, Duration};

async fn start_activity_consumer(
    activity_service: ActivityService,
    kafka_config: &KafkaConfig,
) -> AppResult<()> {
    let consumer = KafkaConsumer::new(kafka_config)?;
    consumer.subscribe(&["rust-server-activity-events"])?;

    // 消费消息
    loop {
        match consumer.recv().await {
            Some(message) => {
                let event: ActivityKafkaEvent = serde_json::from_slice(message.payload())?;
                // 处理事件：发送通知、更新统计等
                println!("Received event: {}", event.description);
            }
            None => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
```

### 添加邮件通知

当特定事件发生时发送邮件：

```rust
if event.event_type == "user_registered" {
    send_welcome_email(event.user_id).await?;
}
```

### 添加数据统计

使用活动数据生成统计报表：

- 每日活跃用户数
- 操作频率分布
- 异常行为检测

## 相关文档

- [Kafka 完整演示文档](./kafka-activity-demo.md)
- [项目主文档](../CLAUDE.md)
- [API 文档](http://localhost:8080/swagger-ui)

## License

MIT
