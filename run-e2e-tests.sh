#!/bin/bash

# Start Server and Run E2E Tests
# This script starts all required services and runs the Playwright tests

set -e

echo "🚀 Starting Rust Server Demo E2E Tests..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install Node.js and npm first."
    exit 1
fi

# Start required services
echo "📦 Starting PostgreSQL and Redis..."
docker-compose up -d postgres redis

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 5

# Check if services are running
if ! docker-compose ps | grep -q "Up"; then
    echo "❌ Failed to start services. Check docker-compose logs."
    exit 1
fi

# Check PostgreSQL
echo "🔍 Checking PostgreSQL connection..."
until docker-compose exec -T postgres pg_isready -U postgres &> /dev/null; do
    echo "   Waiting for PostgreSQL..."
    sleep 1
done
echo "✅ PostgreSQL is ready"

# Check Redis
echo "🔍 Checking Redis connection..."
until docker-compose exec -T redis redis-cli ping &> /dev/null; do
    echo "   Waiting for Redis..."
    sleep 1
done
echo "✅ Redis is ready"

# Set environment variables
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_server"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="test-secret-key-at-least-32-characters-long-for-local-testing"
export RUST_LOG="info"

# Run database migrations
echo "🗄️  Running database migrations..."
cargo install sqlx-cli --no-default-features --features rustls,postgres 2>/dev/null || true
sqlx database create 2>/dev/null || echo "Database already exists"
sqlx migrate run

# Build and start server in background
echo "🔨 Building server..."
cargo build

echo "🚀 Starting server..."
cargo run &
SERVER_PID=$!

# Wait for server to be ready
echo "⏳ Waiting for server to start..."
timeout 30 bash -c 'until curl -s http://localhost:8080/health > /dev/null 2>&1; do sleep 1; done' || {
    echo "❌ Server failed to start within 30 seconds"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
}

echo "✅ Server is running (PID: $SERVER_PID)"

# Run Playwright tests
echo "🧪 Running Playwright E2E tests..."
npm test

TEST_EXIT_CODE=$?

# Cleanup
echo ""
echo "🧹 Cleaning up..."
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

# Stop Docker services
echo "🛑 Stopping Docker services..."
docker-compose down

# Exit with test exit code
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo "✅ All tests passed!"
else
    echo "❌ Some tests failed (exit code: $TEST_EXIT_CODE)"
fi

exit $TEST_EXIT_CODE
