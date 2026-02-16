-- Create activity logs table
CREATE TABLE IF NOT EXISTS activity_logs (
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

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_activity_logs_user_id ON activity_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_activity_logs_event_type ON activity_logs(event_type);
CREATE INDEX IF NOT EXISTS idx_activity_logs_category ON activity_logs(event_type_category);
CREATE INDEX IF NOT EXISTS idx_activity_logs_created_at ON activity_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_activity_logs_user_created ON activity_logs(user_id, created_at DESC);

-- Create index for time-based queries
CREATE INDEX IF NOT EXISTS idx_activity_logs_created_at_desc ON activity_logs(created_at DESC);

-- Add comment
COMMENT ON TABLE activity_logs IS 'Audit log for tracking user and system events';
COMMENT ON COLUMN activity_logs.event_type_category IS 'Category of event: user, message, room, system, error';
COMMENT ON COLUMN activity_logs.metadata IS 'Additional event data in JSON format';
