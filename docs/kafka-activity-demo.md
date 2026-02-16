# Kafka 使用场景演示 - 实时活动日志系统

## 功能概述

本系统展示 Kafka 作为事件流平台的强大功能，实现了一个**实时活动日志系统**，用于跟踪和审计用户操作。

## 架构设计

```
用户操作 → 后端服务 → Kafka Producer → Kafka Topic → Kafka Consumer → PostgreSQL → Redis Cache → 前端展示
```

### 核心组件

1. **活动事件模型** (`src/models/activity.rs`)
   - 定义多种事件类型：用户注册、登录、登出、消息发送等
   - 包含事件元数据、IP地址、User Agent等信息
   - 支持事件分类：user, message, room, system, error

2. **Kafka Producer** (`src/kafka/producer.rs`)
   - 发布活动事件到 Kafka topic: `rust-server-activity-events`
   - 异步发送，支持批量处理
   - 自动重试和错误处理

3. **Activity Service** (`src/services/activity_service.rs`)
   - 创建活动日志并持久化到数据库
   - 将事件发布到 Kafka
   - 缓存最近活动到 Redis（5分钟TTL）
   - 支持分页查询

4. **API Endpoints** (`src/handlers/activity.rs`)
   - `GET /api/activities` - 获取所有活动日志（分页）
   - `GET /api/activities/recent` - 获取最近50条活动（缓存）
   - `GET /api/activities/user/{id}` - 获取特定用户的活动
   - `POST /api/activities` - 创建活动日志

## 使用场景

### 场景 1: 用户审计跟踪

记录所有用户操作，满足安全合规要求：

```rust
// 用户登录时记录
activity_service.create_activity(&CreateActivityLogRequest {
    user_id: Some(user.id),
    event_type: "user_logged_in".to_string(),
    description: "User logged in successfully".to_string(),
    metadata: Some(serde_json::json!({
        "username": user.username,
        "login_method": "password"
    })),
    ip_address: Some(client_ip),
    user_agent: Some(user_agent),
}).await?;
```

### 场景 2: 实时监控仪表盘

前端实时展示系统活动流：

```typescript
// 前端轮询获取最新活动（每5秒自动刷新）
const { activities } = await activityApi.getRecentActivities(50);
```

### 场景 3: 事件驱动架构

Kafka Consumer 可以订阅活动事件，触发其他服务：

```rust
// Kafka Consumer 示例（可选实现）
let consumer = KafkaConsumer::new(&kafka_config)?;
consumer.subscribe(&["rust-server-activity-events"])?;

// 消费事件并发送通知、更新统计等
while let Some(message) = consumer.recv().await {
    let event: ActivityKafkaEvent = serde_json::from_slice(message.payload())?;
    // 处理事件：发送邮件、更新指标、触发工作流等
}
```

### 场景 4: 安全分析

分析异常行为模式：

- 检测暴力破解攻击（多次失败的登录尝试）
- 识别可疑的位置变化
- 监控特权用户操作

## 数据库表结构

```sql
CREATE TABLE activity_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(100) NOT NULL,
    event_type_category VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    metadata JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 索引优化查询性能
CREATE INDEX idx_activity_logs_user_id ON activity_logs(user_id);
CREATE INDEX idx_activity_logs_event_type ON activity_logs(event_type);
CREATE INDEX idx_activity_logs_created_at ON activity_logs(created_at DESC);
```

## 性能优化

1. **缓存策略**
   - Redis 缓存最近50条活动（5分钟TTL）
   - 减少数据库查询压力

2. **批量处理**
   - Kafka Producer 支持批量发送消息
   - 配置：`linger.ms=10`, `batch.size=16384`

3. **异步处理**
   - 活动记录异步写入，不阻塞主业务流程
   - 失败不影响用户操作

## 运行服务

### 启动 Kafka（使用 Docker）

```bash
# 启动 Kafka 和 Zookeeper
docker-compose up -d kafka zookeeper
```

### 启动后端服务（启用 Kafka 功能）

```bash
# 编译并运行，启用 Kafka feature
cargo run --features kafka

# 或使用预编译二进制
export KAFKA_BROKERS=localhost:9092
./target/release/server
```

### 环境变量配置

```bash
# Kafka 配置
KAFKA_BROKERS=localhost:9092
KAFKA_TOPICS=rust-server-activity-events
KAFKA_GROUP_ID=rust-server-consumer-group
```

## 前端界面

访问 `/activities` 页面查看实时活动流：

- 🎨 精美的活动卡片展示
- 🔄 自动刷新（每5秒）
- 🔍 事件分类和图标
- 📊 元数据详情查看
- ⏸️ 暂停/恢复控制

## 扩展建议

1. **添加更多事件类型**
   - 文件上传/下载
   - API 调用记录
   - 支付交易

2. **集成更多消费者**
   - 数据分析服务
   - 告警系统
   - 审计报告生成

3. **增强功能**
   - 事件搜索和过滤
   - 导出为 CSV/JSON
   - 可视化图表
   - 实时统计仪表盘

## Kafka 的优势

1. **解耦** - 活动记录与主业务流程解耦
2. **可扩展** - 轻松添加多个消费者处理不同任务
3. **持久化** - Kafka 保留消息历史，支持重放
4. **高吞吐** - 支持每秒百万级事件处理
5. **容错** - 副本机制保证数据不丢失

## 总结

这个实时活动日志系统展示了 Kafka 在事件驱动架构中的核心作用：

- ✅ **异步事件发布** - 不影响主业务性能
- ✅ **事件持久化** - 数据安全可靠存储
- ✅ **实时监控** - 前端实时展示系统活动
- ✅ **可扩展架构** - 轻松添加新的消费者
- ✅ **审计合规** - 满足安全和合规要求

通过 Kafka，我们实现了从简单日志记录到强大事件驱动平台的转变！
