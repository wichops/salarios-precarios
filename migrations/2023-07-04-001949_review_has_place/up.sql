-- Your SQL goes here
ALTER TABLE reviews
    ADD COLUMN IF NOT EXISTS place_id INTEGER NOT NULL;

ALTER TABLE reviews
    ADD CONSTRAINT fk_reviews_places FOREIGN KEY (place_id) REFERENCES places(id);
