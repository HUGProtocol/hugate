-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  profile_image VARCHAR,
  email VARCHAR,
  twitter VARCHAR,
  about VARCHAR,
  pts bigint DEFAULT 0,
  create_at TIMESTAMPTZ default NOW(),
  updated_at TIMESTAMPTZ default NOW()
)