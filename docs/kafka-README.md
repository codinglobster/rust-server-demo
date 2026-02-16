# Kafka 活动日志系统

这是一个展示 **Kafka** 作为事件流平台的完整演示系统，实现了实时活动日志记录和监控功能。

## 🎯 核心功能

- ✅ **实时事件追踪** - 捕获用户操作（登录、注册、消息等）
- ✅ **Kafka 集成** - 事件发布到 Kafka topic 进行异步处理
- ✅ **高性能缓存** - Redis 缓存最近活动（5分钟TTL）
- ✅ **前端展示** - 实时活动流界面，自动刷新
- ✅ **RESTful API** - 完整的活动日志查询接口
- ✅ **事件分类** - 支持多种事件类型和元数据
- ✅ **分页查询** - 高效的历史记录浏览

## 📋 目录结构

```
rust-server-demo/
├── src/
│   ├── models/
│   │   └── activity.rs              # 活动日志模型
│   ├── kafka/
│   │   ├── mod.rs                   # Kafka 模块
│   │   └── producer.rs              # Kafka Producer
│   ├── services/
│   │   └── activity_service.rs      # 活动服务
│   └── handlers/
│       └── activity.rs              # API 处理器
├── frontend/
│   └── src/
│       ├── lib/
│       │   ├── components/
│       │   │   └── ActivityStream.svelte  # 活动流组件
│       │   ├── services/api/
│       │   │   └── activity.api.ts        # API 客户端
│       │   └── types/
│       │       └── activity.ts            # 类型定义
│       └── routes/
│           └── activities/                 # 活动页面
├── migrations/
│   └── *_create_activity_logs_table.sql    # 数据库迁移
├── docs/
│   ├── kafka-activity-demo.md       # 完整演示文档
│   └── kafka-quick-start.md         # 快速开始指南
└── scripts/
    └── test-kafka-activity.sh       # 测试脚本
```

## 🚀 快速开始

### 1. 启动服务

```bash
# 启动基础设施（PostgreSQL, Redis, Kafka）
docker-compose up -d

# 启动后端（启用 Kafka 功能）
cargo run --features kafka

# 启动前端
cd frontend && npm run dev
```

### 2. 运行测试脚本

```bash
# 自动化测试 Kafka 活动日志功能
./scripts/test-kafka-activity.sh
```

### 3. 访问界面

- **前端活动流**: http://localhost:3000/activities
- **Swagger API**: http://localhost:8080/swagger-ui

## 📚 使用场景

### 1. 用户审计跟踪

记录所有用户操作，满足安全合规要求：

```rust
activity_service.create_activity(&CreateActivityLogRequest {
    user_id: Some(user.id),
    event_type: "user_logged_in".to_string(),
    description: "User logged in from IP".to_string(),
    metadata: Some(json!({"location": "Beijing"})),
    ip_address: Some(client_ip),
    user_agent: Some(user_agent),
}).await?;
```

### 2. 实时监控仪表盘

前端实时展示系统活动，自动刷新：

```typescript
const { activities } = await activityApi.getRecentActivities(50);
```

### 3. 事件驱动架构

Kafka Consumer 订阅活动事件，触发下游服务：

```rust
// Kafka 消费者示例
let consumer = KafkaConsumer::new(&kafka_config)?;
consumer.subscribe(&["rust-server-activity-events"])?;

while let Some(message) = consumer.recv().await {
    let event: ActivityKafkaEvent = serde_json::from_slice(message.payload())?;
    // 处理：发送邮件、更新指标、触发工作流
}
```

### 4. 安全分析

分析异常行为模式：

- 检测暴力破解攻击
- 识别可疑的位置变化
- 监控特权用户操作

## 🏗️ 架构设计

```
┌─────────────┐
│  用户操作   │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────┐
│   Activity Service          │
│  - 持久化到 PostgreSQL       │
│  - 缓存到 Redis              │
│  - 发布到 Kafka              │
└──────┬───────────┬───────────┘
       │           │
       ▼           ▼
┌──────────┐  ┌──────────────┐
│ Database │  │    Kafka     │
└──────────┘  │   Producer   │
              └──────┬───────┘
                     │
                     ▼
              ┌──────────────┐
              │ Kafka Topic   │
              └──────┬───────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
   ┌─────────┐  ┌─────────┐  ┌─────────┐
   │Consumer │  │Consumer │  │Consumer │
   │(邮件)   │  │(统计)   │  │(分析)   │
   └─────────┘  └─────────┘  └─────────┘
```

## 🔌 API 端点

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| GET | `/api/activities/recent` | 获取最近活动（缓存） | 必需 |
| GET | `/api/activities` | 获取所有活动（分页） | 必需 |
| GET | `/api/activities/user/{id}` | 获取用户活动 | 必需 |
| POST | `/api/activities` | 创建活动日志 | 必需 |

## 📊 支持的事件类型

### 用户事件
- `user_registered` - 用户注册
- `user_logged_in` - 用户登录
- `user_logged_out` - 用户登出
- `user_updated` - 信息更新
- `user_password_changed` - 密码修改

### 消息事件
- `message_sent` - 发送消息
- `message_edited` - 编辑消息
- `message_deleted` - 删除消息

### 房间事件
- `room_joined` - 加入房间
- `room_left` - 离开房间
- `room_created` - 创建房间

### 系统事件
- `system_alert` - 系统告警
- `system_notification` - 系统通知

### 错误事件
- `error_occurred` - 错误发生

## 🎨 前端特性

- 🔄 **自动刷新** - 每5秒自动更新
- 📱 **响应式设计** - 支持各种设备
- 🎯 **事件分类** - 图标和颜色区分
- 📝 **元数据查看** - 展开查看详情
- ⏸️ **暂停/恢复** - 控制自动刷新

## ⚡ 性能优化

1. **Redis 缓存**
   - 最近50条活动缓存
   - 5分钟 TTL
   - 减少90%数据库查询

2. **Kafka 批量处理**
   - 批量大小：16KB
   - 延迟：10ms
   - 压缩：snappy

3. **数据库索引**
   - user_id, event_type, created_at
   - 复合索引优化查询

## 📈 监控指标

系统自动记录以下指标：

- 活动创建速率
- Kafka 发布延迟
- 缓存命中率
- API 响应时间

访问 `http://localhost:8080/metrics` 查看 Prometheus 指标。

## 🔧 配置

### 环境变量

```bash
# Kafka 配置
KAFKA_BROKERS=localhost:9092
KAFKA_TOPICS=rust-server-activity-events
KAFKA_GROUP_ID=rust-server-consumer-group
KAFKA_SESSION_TIMEOUT=10000
KAFKA_AUTO_OFFSET_RESET=latest
```

### Docker Compose

```yaml
kafka:
  image: confluentinc/cp-kafka:latest
  ports:
    - "9092:9092"
  environment:
    KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
    KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
```

## 🐛 故障排查

### Kafka 连接失败

```bash
# 检查 Kafka 是否运行
docker ps | grep kafka

# 查看 Kafka 日志
docker logs kafka

# 测试连接
nc -zv localhost 9092
```

### 活动未记录

```bash
# 启用调试日志
RUST_LOG=debug cargo run --features kafka

# 检查数据库
psql -U postgres -d rust_server -c "SELECT * FROM activity_logs ORDER BY created_at DESC LIMIT 10;"
```

## 📖 文档

- [完整演示文档](./kafka-activity-demo.md) - 深入了解 Kafka 集成
- [快速开始指南](./kafka-quick-start.md) - 5分钟上手
- [项目主文档](../CLAUDE.md) - 整体架构说明

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 License

MIT
