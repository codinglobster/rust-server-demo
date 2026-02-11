-- Initialize database script
-- This script is automatically run by Docker when creating the PostgreSQL container

\c rust_server;

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- The actual migrations are handled by the Rust application
-- This script just ensures the database exists
