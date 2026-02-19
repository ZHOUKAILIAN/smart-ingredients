CREATE TABLE IF NOT EXISTS community_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_type VARCHAR(20) NOT NULL CHECK (author_type IN ('anonymous','user')),
    user_id UUID,
    share_token_hash VARCHAR(128),
    summary_text TEXT NOT NULL,
    health_score INTEGER CHECK (health_score >= 0 AND health_score <= 100),
    ingredients_raw TEXT NOT NULL,
    card_payload JSONB NOT NULL,
    card_image_url VARCHAR(512),
    source_analysis_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_community_posts_created_at ON community_posts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_community_posts_user_id ON community_posts(user_id);
