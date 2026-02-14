-- Seed completed analyses for a given user_id (UUID) so the History page has enough items to scroll.
-- Usage:
--   docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients -v ON_ERROR_STOP=1 -v user_id=... < scripts/seed_user_history.sql

INSERT INTO analyses (
  image_url,
  ocr_text,
  confirmed_text,
  ocr_status,
  llm_status,
  ocr_completed_at,
  health_score,
  result,
  status,
  error_message,
  user_id,
  created_at,
  updated_at
)
SELECT
  '',
  NULL,
  NULL,
  'completed',
  'completed',
  NOW() - (gs * INTERVAL '1 minute'),
  (60 + (random() * 40))::int,
  jsonb_build_object('summary', '测试历史记录 ' || gs::text),
  'completed',
  NULL,
  :'user_id'::uuid,
  NOW() - (gs * INTERVAL '1 minute'),
  NOW() - (gs * INTERVAL '1 minute')
FROM generate_series(1, 25) AS gs;

