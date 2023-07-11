-- Your SQL goes here
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  session_token TEXT,
  access_token TEXT,

  user_id INT REFERENCES users(id)
)
