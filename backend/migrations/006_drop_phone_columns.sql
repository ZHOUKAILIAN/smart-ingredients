ALTER TABLE users
    DROP COLUMN IF EXISTS phone_encrypted,
    DROP COLUMN IF EXISTS phone_hash;
