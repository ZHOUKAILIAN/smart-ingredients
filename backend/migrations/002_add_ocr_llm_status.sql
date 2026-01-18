ALTER TABLE analyses
ADD COLUMN IF NOT EXISTS ocr_status VARCHAR(20) NOT NULL DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS ocr_text TEXT,
ADD COLUMN IF NOT EXISTS ocr_completed_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS llm_status VARCHAR(20) NOT NULL DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS confirmed_text TEXT;

CREATE INDEX IF NOT EXISTS idx_analyses_ocr_status ON analyses(ocr_status);
CREATE INDEX IF NOT EXISTS idx_analyses_llm_status ON analyses(llm_status);
