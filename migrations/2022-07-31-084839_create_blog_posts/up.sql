CREATE TABLE flags (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 'f'
)