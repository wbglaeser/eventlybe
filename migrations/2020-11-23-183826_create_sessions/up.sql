-- Your SQL goes here
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  userid TEXT NOT NULL,
  sessionid TEXT NOT NULL
)
