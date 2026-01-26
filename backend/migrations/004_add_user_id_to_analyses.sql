ALTER TABLE analyses
    ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_analyses_user_id ON analyses(user_id);
