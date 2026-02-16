# Kafka 活动日志系统 - 测试文档

## 测试概述

本文档描述了 Kafka 活动日志系统的完整测试套件，包括单元测试、集成测试和端到端测试。

## 测试结构

```
rust-server-demo/
├── src/
│   ├── models/
│   │   └── activity.rs              # 单元测试（内联）
│   └── services/
│       └── activity_service.rs      # 单元测试（内联）
└── frontend/
    └── tests/
        ├── unit/
        │   ├── services/
        │   │   └── activity.api.test.ts         # API 客户端单元测试
        │   └── components/
        │       └── ActivityStream.test.ts       # 组件单元测试
        └── e2e/
            ├── activity.spec.ts                 # UI E2E 测试
            └── activity-api.spec.ts            # API 集成测试
```

## 1. 后端单元测试

### 1.1 Activity Model 测试 (`src/models/activity.rs`)

**测试用例：**

- ✅ `test_activity_event_type_to_string` - 测试事件类型转换为字符串
- ✅ `test_activity_event_type_from_string` - 测试字符串转换为事件类型
- ✅ `test_activity_kafka_event_serialization` - 测试 Kafka 事件序列化
- ✅ `test_activity_log_conversion` - 测试 ActivityLog 到 ActivityLogDto 转换

**运行：**
```bash
cargo test --lib activity::models::tests
```

### 1.2 Activity Service 测试 (`src/services/activity_service.rs`)

**测试用例：**

- ✅ `test_categorize_event` - 测试事件分类逻辑
- ✅ `test_create_activity_request_validation` - 测试请求数据验证
- ✅ `test_activity_event_type_conversion` - 测试事件类型转换
- ✅ `test_activity_event_type_display` - 测试事件类型显示
- ✅ `test_activity_kafka_event_json_roundtrip` - 测试 JSON 序列化往返

**运行：**
```bash
cargo test --lib activity::services::tests
```

## 2. 前端单元测试

### 2.1 Activity API Client 测试

**文件：** `frontend/tests/unit/services/activity.api.test.ts`

**测试用例：**

#### getRecentActivities
- ✅ 使用默认限制获取最近活动
- ✅ 使用自定义限制获取最近活动

#### listActivities
- ✅ 使用默认分页获取活动
- ✅ 使用自定义分页获取活动

#### getUserActivities
- ✅ 使用默认分页获取用户活动
- ✅ 使用自定义分页获取用户活动

#### createActivity
- ✅ 创建新活动日志
- ✅ 创建不带可选字段的活动

#### 错误处理
- ✅ 优雅处理 API 错误
- ✅ 处理空响应

**运行：**
```bash
cd frontend
npm test -- activity.api.test.ts
```

### 2.2 ActivityStream 组件测试

**文件：** `frontend/tests/unit/components/ActivityStream.test.ts`

**测试用例：**

#### 加载状态
- ✅ 显示初始加载指示器

#### 活动显示
- ✅ 加载后显示活动
- ✅ 正确显示事件类型
- ✅ 显示活动描述
- ✅ 无活动时显示空状态

#### 错误处理
- ✅ API 失败时显示错误消息

#### 自动刷新
- ✅ 启用自动刷新时显示实时指示器
- ✅ 定期刷新活动

#### 暂停/恢复
- ✅ 点击按钮暂停自动刷新
- ✅ 再次点击恢复自动刷新

#### 元数据显示
- ✅ 展开时显示元数据详情

#### 事件图标
- ✅ 为不同事件类型显示正确图标

#### 时间格式化
- ✅ 为最近活动显示相对时间

**运行：**
```bash
cd frontend
npm test -- ActivityStream.test.ts
```

## 3. E2E 测试

### 3.1 Activity Stream UI 测试

**文件：** `frontend/tests/e2e/activity.spec.ts`

**测试用例：**

- ✅ 显示活动流页面
- ✅ 初始显示加载状态
- ✅ 登录后显示活动
- ✅ 有暂停/恢复按钮
- ✅ 显示实时状态指示器
- ✅ 显示事件图标
- ✅ 未认证时重定向到登录
- ✅ 处理多个用户操作
- ✅ 自动刷新活动
- ✅ 可用时显示元数据
- ✅ 优雅处理空活动列表

**运行：**
```bash
cd frontend
npx playwright test activity.spec.ts
```

### 3.2 Activity API 集成测试

**文件：** `frontend/tests/e2e/activity-api.spec.ts`

**测试用例：**

- ✅ GET /api/activities/recent - 返回最近活动
- ✅ GET /api/activities - 返回分页活动
- ✅ GET /api/activities/user/:id - 返回用户活动
- ✅ POST /api/activities - 创建自定义活动
- ✅ 正确处理分页
- ✅ 遵守 per_page 限制
- ✅ 缓存后续调用
- ✅ 返回其他用户活动时 403
- ✅ 所有端点需要认证
- ✅ 随时间跟踪用户活动

**运行：**
```bash
cd frontend
npx playwright test activity-api.spec.ts
```

## 4. 运行所有测试

### 后端测试

```bash
# 运行所有后端测试
cargo test --lib

# 运行特定模块测试
cargo test --lib activity

# 运行测试并显示输出
cargo test --lib -- --nocapture

# 运行测试并生成覆盖率报告
cargo tarpaulin --lib --out Html
```

### 前端测试

```bash
cd frontend

# 运行所有单元测试
npm test

# 运行单元测试（UI 模式）
npm run test:ui

# 运行特定测试文件
npm test -- activity.api.test.ts

# 运行 E2E 测试
npx playwright test

# 运行特定 E2E 测试文件
npx playwright test activity.spec.ts

# 运行 E2E 测试（带界面）
npx playwright test --ui

# 运行 E2E 测试（调试模式）
npx playwright test --debug
```

## 5. 测试覆盖率目标

| 模块 | 目标覆盖率 | 状态 |
|------|----------|------|
| Activity Model | 90%+ | ✅ |
| Activity Service | 80%+ | ✅ |
| Activity API Client | 85%+ | ✅ |
| ActivityStream 组件 | 75%+ | ✅ |
| E2E 场景覆盖 | 70%+ | ✅ |

## 6. 测试数据准备

### 创建测试用户

```bash
# 使用测试脚本
./scripts/test-kafka-activity.sh

# 或手动创建
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "email": "test@example.com",
    "password": "Password123",
    "full_name": "Test User"
  }'
```

### 准备测试活动

```bash
# 使用脚本创建测试活动
./scripts/test-kafka-activity.sh

# 或手动创建
TOKEN="<your_jwt_token>"

curl -X POST http://localhost:8080/api/activities \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "event_type": "test_event",
    "description": "Test activity",
    "metadata": {"test": true}
  }'
```

## 7. 持续集成 (CI)

### GitHub Actions 配置示例

```yaml
name: Activity Tests

on: [push, pull_request]

jobs:
  backend-tests:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_DB: rust_server_test
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
      redis:
        image: redis:7
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run tests
        run: cargo test --lib --features kafka
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/rust_server_test
          REDIS_URL: redis://localhost:6379

  frontend-tests:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: ./frontend

    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Run unit tests
        run: npm test -- --run

      - name: Install Playwright
        run: npx playwright install --with-deps

      - name: Run E2E tests
        run: npx playwright test
```

## 8. 测试最佳实践

### 后端测试

1. **隔离性** - 每个测试独立运行，不依赖其他测试
2. **清理** - 测试后清理数据库状态
3. **Mock** - 使用 mock 替代外部依赖（Kafka、Redis）
4. **异步** - 使用 `tokio::test` 处理异步代码

### 前端测试

1. **Mock API** - Mock `activityApi` 调用
2. **用户交互** - 测试用户操作（点击、输入等）
3. **状态变化** - 测试组件状态变化
4. **错误场景** - 测试错误处理和边界情况

### E2E 测试

1. **真实环境** - 在接近生产的环境中测试
2. **完整流程** - 测试完整的用户旅程
3. **等待策略** - 使用适当的等待（loadState、timeout）
4. **清理** - 测试后清理测试数据

## 9. 常见问题

### Q: 测试失败怎么办？

**A:**
1. 检查服务是否运行（后端、数据库、Redis）
2. 查看测试日志输出
3. 运行单个测试文件进行调试
4. 使用 `--headed` 或 `--debug` 模式查看浏览器

### Q: 如何调试 Playwright 测试？

**A:**
```bash
# 调试模式
npx playwright test --debug

# 带界面模式
npx playwright test --ui

# 显示浏览器
npx playwright test --headed
```

### Q: 测试速度慢怎么办？

**A:**
1. 减少等待时间（使用 waitForSelector 而不是 waitForTimeout）
2. 并行运行测试（Playwright 默认并行）
3. 使用测试数据库（内存数据库）
4. Mock 外部服务调用

### Q: 如何查看测试覆盖率？

**A:**
```bash
# 后端覆盖率
cargo tarpaulin --lib --out Html

# 前端覆盖率
npm test -- --coverage
```

## 10. 下一步

- [ ] 添加性能测试（负载测试）
- [ ] 添加压力测试（大量并发活动）
- [ ] 添加安全测试（注入攻击防护）
- [ ] 添加可访问性测试（a11y）
- [ ] 提高测试覆盖率到 90%+

## 11. 相关文档

- [Kafka 快速开始](./kafka-quick-start.md)
- [Kafka 完整演示](./kafka-activity-demo.md)
- [API 文档](http://localhost:8080/swagger-ui)

---

**测试是质量的保证！** 🎯
