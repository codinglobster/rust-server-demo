# Rust Server Demo

A production-ready real-time/long-connection server built with Rust, featuring PostgreSQL for persistence, Redis for caching, and Kafka for message queuing.

## Features

- **RESTful API** with Axum web framework
- **WebSocket support** for real-time communication
- **JWT authentication** with access and refresh tokens
- **PostgreSQL** for data persistence with SQLx
- **Redis** for caching and session management
- **Kafka** for asynchronous message processing
- **OpenAPI documentation** with Swagger UI
- **Structured logging** with tracing
- **Prometheus metrics** for monitoring
- **Graceful shutdown** handling

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    HTTP/WebSocket Layer                 │
│                   (Axum Router + Handlers)              │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                   Middleware Layer                      │
│          (Auth, Logging, Rate Limiting)                 │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                   Service Layer                         │
│           (Business Logic & Orchestration)              │
└─────┬────────────┬────────────┬────────────┬───────────┘
      │            │            │            │
┌─────▼─────┐ ┌───▼────┐ ┌────▼────┐ ┌────▼────────┐
│ Repository│ │ Cache  │ │Messaging│ │  External  │
│  (Postgres)│ │ (Redis)│ │ (Kafka) │ │  Services  │
└───────────┘ └────────┘ └─────────┘ └─────────────┘
```

## Project Structure

```
rust-server-demo/
├── config/          # Configuration modules
├── src/
│   ├── core/        # Error handling, telemetry
│   ├── models/      # Data models
│   ├── database/    # Database layer, migrations
│   ├── cache/       # Redis client
│   ├── messaging/   # Kafka producer/consumer
│   ├── auth/        # JWT, middleware
│   ├── services/    # Business logic
│   ├── handlers/    # HTTP handlers
│   ├── routes/      # API routes
│   ├── websocket/   # WebSocket server
│   └── utils/       # Utility functions
├── migrations/      # Database migrations
└── scripts/         # Scripts and configs
```

## Quick Start

### Prerequisites

- Rust 1.70+ (use `rustup` to install)
- Docker and Docker Compose

### Using Docker Compose (Recommended)

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-server-demo
```

2. Start all services:
```bash
docker-compose up -d
```

3. Copy environment variables:
```bash
cp .env.example .env
```

4. Build and run:
```bash
cargo build --release
cargo run --bin server
```

### Manual Setup

1. Start PostgreSQL, Redis, and Kafka using Docker Compose:
```bash
docker-compose up -d postgres redis kafka
```

2. Set environment variables:
```bash
export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/rust_server
export REDIS_URL=redis://localhost:6379
export KAFKA_BROKERS=localhost:9092
export JWT_SECRET=your-secret-key-at-least-32-characters
```

3. Run migrations:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

4. Run the server:
```bash
cargo run
```

## API Documentation

Once the server is running, visit:
- **Swagger UI**: http://localhost:8080/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api-docs/openapi.json
- **Health Check**: http://localhost:8080/health
- **Metrics**: http://localhost:8080/metrics

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login with credentials
- `POST /api/auth/refresh` - Refresh access token
- `POST /api/auth/logout` - Logout current user

### Users
- `GET /api/users/me` - Get current user profile
- `PUT /api/users/me` - Update current user
- `GET /api/users/:id` - Get user by ID
- `GET /api/users` - List users with pagination
- `POST /api/users/me/password` - Change password
- `PUT /api/users/:id/role` - Update user role (admin)

### WebSocket
- `WS /ws` - WebSocket connection for real-time messages

## Configuration

Configuration is done via environment variables. See `.env.example` for all available options.

Key configuration variables:
- `SERVER_HOST` - Server host (default: 0.0.0.0)
- `SERVER_PORT` - Server port (default: 8080)
- `DATABASE_URL` - PostgreSQL connection string
- `REDIS_URL` - Redis connection string
- `KAFKA_BROKERS` - Kafka broker addresses
- `JWT_SECRET` - Secret key for JWT signing (must be at least 32 characters)

## Testing

Run tests:
```bash
cargo test
```

Run with coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Development

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Database Migrations

Create a new migration:
```bash
sqlx migrate add migration_name
```

Run migrations:
```bash
sqlx migrate run
```

## Monitoring

- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin)
- **Jaeger**: http://localhost:16686

## Deployment

1. Set environment variables for production:
```bash
export APP_ENV=production
export JWT_SECRET=<your-secure-secret>
export DATABASE_URL=<production-database-url>
export REDIS_URL=<production-redis-url>
export KAFKA_BROKERS=<production-kafka-brokers>
```

2. Build the release binary:
```bash
cargo build --release
```

3. Run the server:
```bash
./target/release/server
```

## License

MIT License

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
