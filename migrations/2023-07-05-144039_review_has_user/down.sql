-- This file should undo anything in `up.sql`
ALTER TABLE reviews DROP CONSTRAINT fk_reviews_users;

ALTER TABLE reviews DROP COLUMN user_id;
