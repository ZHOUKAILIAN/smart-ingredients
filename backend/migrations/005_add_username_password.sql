ALTER TABLE users
    ADD COLUMN IF NOT EXISTS username TEXT,
    ADD COLUMN IF NOT EXISTS username_normalized TEXT,
    ADD COLUMN IF NOT EXISTS password_hash TEXT,
    ADD COLUMN IF NOT EXISTS password_updated_at TIMESTAMPTZ;

ALTER TABLE users
    ALTER COLUMN phone_encrypted DROP NOT NULL,
    ALTER COLUMN phone_hash DROP NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username_normalized
    ON users (username_normalized);
