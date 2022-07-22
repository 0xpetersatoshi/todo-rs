-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  todo_name VARCHAR NOT NULL,
  is_complete BOOLEAN NOT NULL DEFAULT 'f'
)