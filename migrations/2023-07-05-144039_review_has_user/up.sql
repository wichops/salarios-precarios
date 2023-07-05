-- Your SQL goes here
ALTER TABLE reviews
    ADD COLUMN IF NOT EXISTS user_id INTEGER NOT NULL;

ALTER TABLE reviews
    ADD CONSTRAINT fk_reviews_users FOREIGN KEY (user_id) REFERENCES users(id);
