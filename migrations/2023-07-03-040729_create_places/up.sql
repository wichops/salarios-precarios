-- Your SQL goes here
CREATE TABLE places (
  id SERIAL PRIMARY KEY,
  name VARCHAR(80) NOT NULL,
  address TEXT,
  maps_url TEXT
)
