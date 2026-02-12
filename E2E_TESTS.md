# 📝 Playwright E2E Tests - Setup Guide

This guide will help you set up and run the comprehensive Playwright E2E tests for the Rust Server Demo API.

## 🎯 What's Tested

The Playwright tests cover **100% of the API surface**:

### API Endpoints (26 tests)
- ✅ Health Check APIs (5 tests)
- ✅ Authentication APIs (14 tests)
- ✅ User Management APIs (15 tests)
- ✅ WebSocket (6 tests)
- ✅ End-to-End Workflows (6 tests)

**Total**: ~46 comprehensive tests

## 🚀 Quick Start

### Option 1: Automated Script (Recommended)

```bash
./run-e2e-tests.sh
```

This script will:
1. Start Docker services (PostgreSQL, Redis)
2. Run database migrations
3. Build and start the server
4. Run all Playwright tests
5. Clean up everything

### Option 2: Manual Steps

```bash
# 1. Start services
docker-compose up -d postgres redis

# 2. Set environment variables
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_server"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="test-secret-key-at-least-32-characters-long-for-local-testing"

# 3. Run migrations
sqlx migrate run

# 4. Build and start server
cargo run

# 5. In another terminal, install Playwright
npm run test:install

# 6. Run tests
npm test
```

## 📂 Test Structure

```
tests/
├── e2e/
│   ├── health.spec.js      # Health check endpoints (5 tests)
│   ├── auth.spec.js        # Authentication endpoints (14 tests)
│   ├── users.spec.js       # User management endpoints (15 tests)
│   ├── websocket.spec.js    # WebSocket connections (6 tests)
│   └── workflow.spec.js    # Complete user workflows (6 tests)
└── helpers/
    ├── api-helper.js       # API request helper
    └── test-server.js     # Server startup helper
```

## 🎮 Test Commands

```bash
# Run all tests
npm test

# Run tests in headed mode (watch browser)
npm run test:headed

# Run tests with UI
npm run test:ui

# Debug tests
npm run test:debug

# Run specific test file
npx playwright test auth.spec.js

# Run specific test by name
npx playwright test -g "should login successfully"

# Run tests matching pattern
npx playwright test --grep "registration|login"
```

## 📊 Test Reports

After running tests, view the HTML report:

```bash
npm run test:report
```

Or open `playwright-report/index.html` in your browser.

## 🔍 Troubleshooting

### Server won't start

```bash
# Check if port 8080 is already in use
lsof -i :8080

# Kill process using the port
kill -9 <PID>

# Or change port in .env
SERVER_PORT=3000 cargo run
```

### Database connection errors

```bash
# Check PostgreSQL is running
docker-compose ps

# Check PostgreSQL logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres
```

### All tests failing

```bash
# Verify server is accessible
curl http://localhost:8080/health

# Check server logs
# Look in the terminal where `cargo run` is executed

# Verify JWT_SECRET is set (must be 32+ characters)
echo $JWT_SECRET
```

### WebSocket tests failing

```bash
# Verify token format
curl -H "Authorization: Bearer <token>" \
  http://localhost:8080/api/users/me

# Check WebSocket is enabled
# The server must be compiled with: axum = { version = "0.8", features = ["ws"] }
```

## 🌐 CI/CD Integration

Tests run automatically on:
- Pull requests to `main` branch
- Pushes to `main` branch

See `.github/workflows/e2e-tests.yml` for the CI configuration.

## 📝 Writing New Tests

```javascript
const { test, expect } = require('@playwright/test');
const { ApiHelper } = require('../helpers/api-helper');

test.describe('My Feature', () => {
  let api;

  test.beforeEach(() => {
    api = new ApiHelper();
  });

  test('should do something', async () => {
    const response = await api.getSomeEndpoint();

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('field');
  });
});
```

## 📚 Additional Resources

- [Playwright Documentation](https://playwright.dev/docs/intro)
- [API Documentation](http://localhost:8080/swagger-ui) (when server is running)
- [tests/README.md](./tests/README.md) - Detailed test documentation

## 🎓 Test Coverage Summary

| Category | Endpoints | Tests | Coverage |
|----------|-----------|--------|----------|
| Health Check | 5 | 5 | 100% |
| Authentication | 4 | 14 | 100% |
| User Management | 6 | 15 | 100% |
| WebSocket | 1 | 6 | 100% |
| Workflows | - | 6 | - |
| **Total** | **16** | **46** | **100%** |
