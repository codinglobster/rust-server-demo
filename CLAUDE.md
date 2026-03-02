# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build & Run
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release binary
- `cargo run` - Run debug build
- `./target/release/server` - Run release binary

### Testing
- `cargo test` - Run all tests (34 tests including Redis cache, rate limiting)
- `cargo test --verbose` - Run tests with detailed output
- `cargo tarpaulin --out Html` - Run tests with coverage (requires cargo-tarpaulin)

### E2E Testing (Playwright)
- `npm test` - Run all Playwright E2E tests
- `npm run test:headed` - Run tests in headed mode (see browser)
- `npm run test:ui` - Run tests with Playwright UI
- `npm run test:debug` - Run tests in debug mode
- `npm run test:install` - Install Playwright browsers
- `npm run test:report` - View test report

**Note**: E2E tests require the server to be running first. See `tests/README.md` for details.

### Linting & Formatting
- `cargo fmt` - Format code
- `cargo clippy` - Run linter
- `cargo clippy --fix` - Auto-fix clippy warnings

### Database
- `sqlx database create` - Create database
- `sqlx migrate add <name>` - Create new migration
- `sqlx migrate run` - Run migrations
- `cargo install sqlx-cli` - Install SQLx CLI (if not installed)

### Infrastructure
- `docker-compose up -d` - Start all services (PostgreSQL, Redis, Kafka, Jaeger, Prometheus, Grafana)
- `docker-compose up -d postgres redis kafka` - Start only core services
- `docker-compose down` - Stop all services

### Feature Flags
- Default: No Kafka support
- `--features kafka` - Enable Kafka messaging support

## High-Level Architecture

This is a layered service architecture built with Axum, following a strict separation of concerns:

```
HTTP/WebSocket Layer (Axum Router + Handlers)
         ↓
Middleware Layer (Auth, RateLimit, CORS, Timeout, Trace, CatchPanic)
         ↓
Service Layer (Business Logic & Orchestration)
         ↓
Repository/Cache/Messaging (PostgreSQL, Redis, Kafka)
```

### Core Infrastructure

- **Entry Point**: `src/main.rs` (~300 lines) - Initializes all services, builds router, handles graceful shutdown, cache warmup
- **Error Handling**: `src/core/error.rs` - `AppError` enum, `AppResult<T>` type alias, implements `IntoResponse` for automatic HTTP conversion
- **Telemetry**: `src/core/telemetry.rs` - Structured logging with `tracing`, Prometheus metrics, OpenTelemetry/Jaeger integration
- **State**: `src/state.rs` - `AppState` struct containing all services, implements `FromRef` for Axum sub-state extraction

### Configuration Management

All configuration is environment-variable based (no config files):
- `src/config/base.rs` - ServerConfig (host, port, CORS, compression, rate limiting, timeouts)
- `src/config/database.rs` - DatabaseConfig (pool sizes, timeouts, auto-migrate)
- `src/config/jwt.rs` - JwtConfig (secret, expiration)
- `src/config/redis.rs` - RedisConfig (reconnection, pool size)
- `src/config/kafka.rs` - KafkaConfig (optional, feature-gated)

Configuration is loaded in `main.rs` via `load_*_config()` functions using `std::env::var().unwrap_or_else()`.

### Database Layer

- **Connection**: `src/database/connection.rs` - `Database` struct wrapping `PgPool` with health checks and auto-migration
- **Repositories**: `src/database/repositories/` - Data access using Repository pattern
  - `user.rs` - UserRepository (CRUD operations)
  - `session.rs` - SessionRepository (session management)
  - `message.rs` - MessageRepository (message storage)
  - `activity.rs` - ActivityRepository (activity logging)

Pattern: Repository methods return `AppResult<Option<T>>` for optional results, `AppResult<T>` for required.

### Cache Layer

- **Client**: `src/cache/client.rs` - Redis client wrapper with connection manager and distributed locking
- **Keys**: `src/cache/keys.rs` - `CacheKeys` helpers for structured key generation
- **Distributed Lock**: Full implementation with `acquire_lock`, `release_lock`, `try_lock_with_timeout`

**Cache Improvements** (see `docs/REDIS_IMPROVEMENTS.md`):
- ✅ **Cache Penetration Protection**: Null value caching with 5-minute TTL
- ✅ **Cache Breakdown Protection**: Distributed locks prevent hot-key stampede
- ✅ **Cache Avalanche Protection**: Random TTL jitter (base + 0-300s)
- ✅ **Delayed Double Delete**: Prevents dirty reads in high concurrency
- ✅ **Cache Warmup**: Preloads hot data on startup

Pattern: Cache-aside with Redis → DB fallback, cache invalidation on writes with delayed double delete.

### Authentication

- **JWT**: `src/auth/jwt.rs` - `JwtService` with `TokenPair` (access + refresh tokens), HS256 algorithm
- **Claims**: `src/auth/claims.rs` - JWT Claims struct with user info, roles, expiration
- **Middleware**: `src/auth/middleware.rs` - Authentication middleware for Axum
  - `auth_middleware` - Requires valid JWT
  - `optional_auth_middleware` - Attach user if token present
  - `require_admin_middleware` - Role-based authorization
  - `AuthenticatedUser` extractor for handlers

Pattern: JWT validated in middleware, `TokenUser` injected into request extensions for handlers.

### Middleware Layer

- **Rate Limiting**: `src/middleware/rate_limit.rs` - Redis-based rate limiting
  - Per-user or per-IP request tracking
  - Configurable limits and time windows
  - Returns `X-RateLimit-*` headers
- **Authentication**: Token validation and role-based access control
- **Request Limits**: Body size limit (configurable MB) and timeout (configurable seconds)
- **Other**: CORS, compression, tracing, panic catching

### Models

- **User**: `src/models/user.rs` - User, UserDto, RegisterRequest, LoginRequest, UpdateUserRequest, `UserRole` enum
- **Session**: `src/models/session.rs` - Session, LoginResponse, RefreshTokenRequest
- **Message**: `src/models/message.rs` - WebSocket message models
- **Activity**: `src/models/activity.rs` - Activity log models and event types

Pattern: Separate internal models (with `password_hash`) from DTOs (without sensitive data). Validation via `validator` crate, OpenAPI schemas via `utoipa::ToSchema`.

### Services

Business logic layer orchestrating repositories and cache:
- `src/services/user_service.rs` - User management with caching, distributed locks, cache warmup
- `src/services/auth_service.rs` - Password hashing/verification, login, session management, token validation
- `src/services/session_service.rs` - Session CRUD (⚠️ not exposed via HTTP yet)
- `src/services/message_service.rs` - Message handling with optional Kafka publishing (⚠️ not exposed via HTTP yet)
- `src/services/activity_service.rs` - Activity logging and querying

Pattern: Services are thin wrappers around repositories with caching, distributed locking, and cross-cutting concerns.

### Handlers

HTTP request handlers organized by domain:
- `src/handlers/auth.rs` - register, login, refresh_token, logout
- `src/handlers/user.rs` - get_me, update_user, get_user, list_users, change_password, update_user_role
- `src/handlers/health.rs` - health_check, liveness, readiness, version
- `src/handlers/activity.rs` - list_activities, get_recent_activities, list_user_activities
- `src/handlers/ws.rs` - WebSocket connection handler

Pattern: Handlers use `State<T>` extractor to get services, return `Result<T, impl IntoResponse>`.

### Routes

- `src/routes/api.rs` - API route organization with middleware applied at appropriate levels
- `src/routes/ws.rs` - WebSocket route at `/ws`

### WebSocket

Real-time communication using unbounded MPSC channels:
- `src/websocket/server.rs` - WebSocketServer container
- `src/websocket/connection.rs` - ConnectionManager tracking active connections, room membership, broadcasts
- `src/websocket/broadcast.rs` - Broadcaster for message distribution
- `src/websocket/message.rs` - Message handling logic

Pattern: Redis-backed room membership for multi-server coordination. ⚠️ Room broadcast logic is basic (echo mode).

## Key Patterns & Conventions

### Error Handling
- Use `AppError` enum for all application errors
- Services return `AppResult<T>` = `Result<T, AppError>`
- Handlers return `Result<impl IntoResponse, impl IntoResponse>`
- Errors automatically convert to HTTP responses via `IntoResponse` trait

### Database Access
- Repository pattern for data access
- SQLx with compile-time query verification
- Connection pooling via `PgPool`
- Transactions via `pool.begin().await?`

### Caching Strategy
- **Pattern**: Cache-aside with delayed double delete
- **Read**: Try cache → check null marker → DB → update cache with random TTL
- **Write**: Delete cache → update DB → async delayed delete (500ms)
- **Protection**: Distributed locks prevent cache stampede
- **TTL**: Base + random jitter (e.g., 3600 + 0-300 seconds)
- **Warmup**: Preload hot data on startup (`CACHE_WARMUP_COUNT` env var)

### Rate Limiting
- **Implementation**: Redis-based sliding window
- **Identifier**: User ID (authenticated) or IP address
- **Configuration**: `RATE_LIMIT_REQUESTS` (default: 100), `RATE_LIMIT_WINDOW` (default: 60s)
- **Headers**: Returns `X-RateLimit-Limit` and `X-RateLimit-Remaining`

### Authentication Flow
1. User submits credentials to `/api/auth/login`
2. AuthService verifies password with bcrypt
3. JwtService generates TokenPair (access + refresh)
4. Access token cached in Redis for quick validation (with random TTL)
5. Client includes Bearer token in Authorization header
6. Middleware validates token, injects TokenUser into extensions
7. Handlers extract TokenUser via `AuthenticatedUser` extractor

### Testing Approach
- Unit tests in module `#[cfg(test)]` blocks
- 34 tests covering models, services, utilities, middleware
- Focus on validation logic, error handling, model parsing
- Test placeholders in repositories (need integration tests)
- GitHub Actions runs `cargo build` and `cargo test`

### Async Runtime
- Tokio 1.x with `#[tokio::main]`
- Multi-threaded scheduler
- Graceful shutdown handling (Ctrl+C, SIGTERM)

### Security Patterns
- Passwords hashed with bcrypt (cost: DEFAULT)
- JWT signed with HS256 (secret from env)
- SQL injection prevention via parameterized queries (SQLx)
- CORS configuration in main.rs
- Rate limiting prevents abuse
- Request body size limits
- Request timeouts

### Observability
- Structured logging with `tracing::info!`, `debug!`, `error!`
- Request tracing with span creation
- Prometheus metrics: HTTP requests, WebSocket connections/messages
- Optional Jaeger distributed tracing
- Rate limit warnings logged

### Feature Flags
- `kafka` feature (optional) - Conditionally compiled Kafka support
- Pattern: `#[cfg(feature = "kafka")]` attributes throughout codebase

## Adding a New Feature

1. Create model in `models/`
2. Add repository methods in `database/repositories/`
3. Create service in `services/` (orchestrate repos + cache)
4. Add handler in `handlers/`
5. Register route in `routes/api.rs`
6. Add OpenAPI documentation via `#[utoipa::path]`

## Adding Configuration

1. Add field to appropriate config struct in `config/`
2. Add env var loading in `main.rs` load_*_config()
3. Add to `.env.example`

## API Endpoints

### Health & Monitoring
- `GET /` - Root info with links
- `GET /health` - Health check (DB + Redis status)
- `GET /api/health/health` - Detailed health
- `GET /api/health/liveness` - Liveness probe
- `GET /api/health/readiness` - Readiness probe
- `GET /metrics` - Prometheus metrics

### Authentication (no auth required)
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login, receive tokens
- `POST /api/auth/refresh` - Refresh access token
- `POST /api/auth/logout` - Logout (invalidate token)

### Users (authentication required)
- `GET /api/users/me` - Get current user profile
- `PUT /api/users/me` - Update current user
- `POST /api/users/me/password` - Change password
- `GET /api/users/:id` - Get user by ID
- `GET /api/users/` - List users (pagination)
- `PUT /api/users/:id/role` - Update role (admin only)

### Activity Logs (authentication required)
- `GET /api/activities` - List all activities (with pagination)
- `GET /api/activities/recent` - Get recent activities
- `GET /api/activities/user/:user_id` - Get activities for specific user

### WebSocket
- `WS /ws` - WebSocket connection for real-time messages

## Environment Configuration

### Key Environment Variables

**Server:**
- `SERVER_HOST` - Bind address (default: 0.0.0.0)
- `SERVER_PORT` - Port (default: 8080)
- `APP_ENV` - Environment (development/production)
- `RUST_LOG` - Log level (info/debug/trace)
- `SERVER_MAX_BODY_SIZE` - Max request body size in MB (default: 10)
- `SERVER_REQUEST_TIMEOUT` - Request timeout in seconds (default: 30)

**Rate Limiting:**
- `RATE_LIMIT_REQUESTS` - Max requests per window (default: 100)
- `RATE_LIMIT_WINDOW` - Time window in seconds (default: 60)

**Cache:**
- `CACHE_WARMUP_COUNT` - Number of users to preload on startup (default: 100, set to 0 to disable)

**Database:**
- `DATABASE_URL` - PostgreSQL connection string
- `DB_MAX_CONNECTIONS` - Pool size (default: 10)
- `DB_AUTO_MIGRATE` - Run migrations on startup (default: true)

**Redis:**
- `REDIS_URL` - Redis connection string
- `REDIS_DEFAULT_EXPIRATION` - Default TTL (default: 3600s)

**JWT:**
- `JWT_SECRET` - Signing key (must be 32+ chars)
- `JWT_ACCESS_EXPIRATION` - Access token TTL (default: 3600s)
- `JWT_REFRESH_EXPIRATION` - Refresh token TTL (default: 604800s = 7 days)

**Kafka (optional):**
- `KAFKA_BROKERS` - Comma-separated broker addresses
- `KAFKA_TOPICS` - Topics to publish/consume

## Important Notes

- JWT secret must be 32+ characters or server will fail to start
- Database and Redis must be running before server starts
- Migrations run automatically if `DB_AUTO_MIGRATE=true`
- Metrics global singleton must not be re-registered
- WebSocket room broadcast is basic (echo mode) - full room logic not yet implemented
- Session and message services exist but no HTTP API endpoints yet
- Room management system not yet implemented

## Recent Improvements (2026-03-02)

### Redis Cache Enhancements
See `docs/REDIS_IMPROVEMENTS.md` for full details:
- Implemented distributed locking to prevent cache breakdown
- Added null value caching to prevent cache penetration
- Implemented random TTL jitter to prevent cache avalanche
- Added delayed double delete strategy for write operations
- Implemented cache warmup on service startup

### Middleware Additions
- Rate limiting middleware with Redis backend
- Request body size limiting
- Request timeout middleware
- All configurable via environment variables

### Configuration Updates
- Added `rate_limit_requests` and `rate_limit_window` to ServerConfig
- Added `CACHE_WARMUP_COUNT` environment variable

## Pending Features

See `docs/TODO.md` for complete list. High-priority items:
- ⚠️ WebSocket room broadcast logic (currently echo only)
- ⚠️ Session management HTTP API (service exists, no endpoints)
- ⚠️ Message HTTP API (service exists, no endpoints)
- ⚠️ Room management system (not implemented)
- ⚠️ Email verification flow
- ⚠️ Repository integration tests

## Infrastructure Services

### Docker Compose Services
- **PostgreSQL 16** - Port 5432
- **Redis 7** - Port 6379
- **Kafka + Zookeeper** - Ports 9092 (broker), 2181 (Zookeeper)
- **Jaeger** - Port 16686 (UI), distributed tracing
- **Prometheus** - Port 9090, metrics collection
- **Grafana** - Port 3000, metrics visualization (admin/admin)
