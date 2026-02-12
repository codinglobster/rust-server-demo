# Playwright E2E Tests

This directory contains end-to-end tests for the Rust Server Demo API using Playwright.

## Prerequisites

Before running the tests, ensure:

1. **Start the server**:
   ```bash
   # Start infrastructure services
   docker-compose up -d postgres redis

   # Build and run the server
   cargo run
   ```

2. **Install Playwright browsers**:
   ```bash
   npm run test:install
   ```

## Test Structure

```
tests/
├── e2e/
│   ├── health.spec.js      # Health check endpoints
│   ├── auth.spec.js        # Authentication endpoints
│   ├── users.spec.js       # User management endpoints
│   ├── websocket.spec.js    # WebSocket connections
│   └── workflow.spec.js    # Complete user workflows
└── helpers/
    └── api-helper.js       # API request helper class
```

## Running Tests

### Run all tests
```bash
npm test
```

### Run tests in headed mode (see browser)
```bash
npm run test:headed
```

### Run tests with UI
```bash
npm run test:ui
```

### Debug tests
```bash
npm run test:debug
```

### View test report
```bash
npm run test:report
```

### Run specific test file
```bash
npx playwright test auth.spec.js
```

### Run specific test
```bash
npx playwright test -g "should login successfully"
```

## Test Coverage

### Health Check APIs (`/api/health/*`)
- ✅ GET /health
- ✅ GET /api/health/health
- ✅ GET /api/health/liveness
- ✅ GET /api/health/readiness
- ✅ GET /api/health/version

### Authentication APIs (`/api/auth/*`)
- ✅ POST /api/auth/register
  - User registration
  - Duplicate username handling
  - Validation (email format, password length, username length)
- ✅ POST /api/auth/login
  - Valid credentials
  - Invalid username
  - Invalid password
  - Missing credentials
- ✅ POST /api/auth/refresh
  - Valid refresh token
  - Invalid refresh token
- ✅ POST /api/auth/logout
  - Successful logout
  - Unauthorized access

### User Management APIs (`/api/users/*`)
- ✅ GET /api/users/me
  - Get current user profile
  - Unauthorized access
- ✅ GET /api/users/:id
  - Get user by ID
  - Non-existent user (404)
  - Unauthorized access
- ✅ GET /api/users
  - List users with pagination
  - Custom pagination parameters
  - Unauthorized access
- ✅ PUT /api/users/me
  - Update profile
  - Partial updates
  - Validation errors
  - Unauthorized access
- ✅ POST /api/users/me/password
  - Change password successfully
  - Wrong old password
  - Short new password
  - Login with new password
  - Unauthorized access
- ✅ PUT /api/users/:id/role (Admin only)
  - Update to moderator
  - Update to admin
  - Invalid role
  - Non-admin user (403)
  - Unauthorized access
  - Non-existent user (404)

### WebSocket (`/ws`)
- ✅ Connection with authentication
- ✅ Welcome message
- ✅ Send and receive messages
- ✅ Connection without authentication (should fail)
- ✅ Multiple concurrent connections
- ✅ Connection stability with heartbeat

### Workflows
- ✅ Complete user journey (register → profile → update → logout → login → password change)
- ✅ Token refresh flow
- ✅ Concurrent user operations
- ✅ Error recovery scenarios
- ✅ Data consistency across operations

## Environment Variables

- `BASE_URL` - Server URL (default: `http://localhost:8080`)

Example:
```bash
BASE_URL=http://localhost:3000 npm test
```

## Test Data

Tests use timestamp-based unique data to avoid conflicts:
- Usernames: `testuser_<timestamp>`
- Emails: `testuser_<timestamp>@example.com`

## Cleanup

Tests attempt to clean up after themselves:
- Logout after authentication tests
- Close WebSocket connections after WebSocket tests
- Clear authentication state between tests

## Troubleshooting

### Server not responding
```bash
# Check if server is running
curl http://localhost:8080/health

# Check server logs
# (Look for errors in the terminal where `cargo run` is executed)
```

### Database connection errors
```bash
# Check PostgreSQL is running
docker-compose ps

# Restart PostgreSQL
docker-compose restart postgres
```

### Redis connection errors
```bash
# Check Redis is running
redis-cli ping

# Should return: PONG
```

### WebSocket connection failures
```bash
# Verify JWT_SECRET is set (32+ characters)
echo $JWT_SECRET

# Verify token is valid
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/users/me
```

## CI/CD Integration

Add to your CI pipeline:

```yaml
- name: Start services
  run: docker-compose up -d postgres redis

- name: Run server
  run: cargo run --release &
  env:
    DATABASE_URL: postgresql://postgres:postgres@localhost:5432/rust_server
    REDIS_URL: redis://localhost:6379
    JWT_SECRET: test-secret-key-at-least-32-characters-long

- name: Install Playwright
  run: npm ci && npm run test:install

- name: Run tests
  run: npm test
```
