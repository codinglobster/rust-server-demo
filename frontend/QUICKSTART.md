# 前端快速开始指南

本指南将帮助您快速启动和使用前端应用的新功能。

## 📦 安装依赖

```bash
cd frontend
npm install
```

## 🚀 启动开发服务器

```bash
npm run dev
```

访问：http://localhost:5173

## 🎯 主要功能

### 1. 聊天室 💬

**路由**: `/rooms`

- 查看所有聊天室
- 创建新房间
- 加入房间聊天
- 实时消息更新

**使用步骤**：
1. 点击侧边栏"聊天室"
2. 点击"创建房间"按钮
3. 填写房间信息并创建
4. 点击房间卡片进入聊天
5. 发送消息实时同步

### 2. 会话管理 🔐

**路由**: `/sessions`

- 查看所有活跃会话
- 删除可疑会话
- 批量删除其他会话
- 保护账户安全

**使用步骤**：
1. 点击侧边栏"会话管理"
2. 查看所有登录设备
3. 删除不认识的会话
4. 或一键删除所有其他会话

### 3. 用户管理 👥

**路由**: `/users`

- 查看用户列表
- 查看个人资料
- 编辑个人信息
- 修改密码

### 4. 活动日志 📋

**路由**: `/activities`

- 查看系统活动
- 追踪用户操作
- 分页浏览

## 🛠️ 开发

### 构建生产版本

```bash
npm run build
```

### 预览生产版本

```bash
npm run preview
```

### 运行测试

```bash
npm test
```

### 运行 E2E 测试

```bash
npm run test:e2e
```

## 🔧 配置

### 环境变量

创建 `.env` 文件：

```bash
VITE_API_URL=http://localhost:8080
```

## 📁 项目结构

```
frontend/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── chat/          # 聊天组件
│   │   │   ├── rooms/         # 房间组件
│   │   │   ├── sessions/      # 会话组件
│   │   │   └── ui/            # UI 组件
│   │   ├── services/
│   │   │   ├── api/           # API 客户端
│   │   │   ├── stores/        # Svelte stores
│   │   │   └── websocket.ts   # WebSocket 客户端
│   │   ├── types/             # TypeScript 类型
│   │   └── utils/             # 工具函数
│   └── routes/                # 路由页面
│       ├── rooms/             # 聊天室
│       ├── sessions/          # 会话管理
│       ├── users/             # 用户管理
│       └── activities/        # 活动日志
├── tests/                     # 测试文件
└── package.json
```

## 💡 使用技巧

### 1. 实时聊天

WebSocket 会自动连接，如果断开会自动重连（最多5次）。

### 2. 网络状态

离线时顶部会显示红色横幅提示。

### 3. 通知系统

- 成功操作显示绿色通知
- 错误显示红色通知
- 警告显示黄色通知
- 信息显示蓝色通知

### 4. 表单验证

所有表单都有客户端验证，会实时显示错误提示。

## 🐛 故障排除

### WebSocket 无法连接

检查：
1. 后端服务是否启动
2. 后端地址是否正确（`.env` 文件）
3. Token 是否有效

### API 请求失败

检查：
1. 后端 API 是否已实现
2. Token 是否过期
3. 网络连接是否正常

### 页面空白

检查：
1. 浏览器控制台错误信息
2. 是否已登录
3. 路由是否正确

## 📚 更多文档

- [完整功能文档](../docs/FRONTEND_COMPLETION.md)
- [API 文档](../docs/TODO.md)
- [后端文档](../CLAUDE.md)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可

MIT License
