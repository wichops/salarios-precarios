-- Your SQL goes here
CREATE TABLE reviews (
  id SERIAL PRIMARY KEY,
  weekly_salary REAL NOT NULL,
  weekly_tips REAL,
  shift_days_count INTEGER NOT NULL,
  shift_duration INTEGER NOT NULL,
  social_security BOOLEAN DEFAULT FALSE
)
