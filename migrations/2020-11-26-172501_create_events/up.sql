-- Your SQL goes here
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  userid TEXT NOT NULL,
  name TEXT NOT NULL,
  date TEXT NOT NULL,
  location TEXT NOT NULL
)
