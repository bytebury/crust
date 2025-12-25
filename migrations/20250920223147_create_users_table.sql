CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT,
    full_name TEXT NOT NULL,
    image_url TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    stripe_customer_id TEXT UNIQUE DEFAULT NULL,
    country_id INTEGER REFERENCES countries(id) DEFAULT NULL,
    verified BOOLEAN NOT NULL DEFAULT 0,
    locked BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);

CREATE VIEW users_view AS
SELECT u.id,
       u.email,
       u.first_name,
       u.last_name,
       u.full_name,
       u.image_url,
       u.role,
       u.stripe_customer_id,
       u.country_id,
       u.verified,
       u.locked,
       u.created_at,
       u.updated_at,
       c.name as country_name,
       c.code as country_code,
       c.locked as country_locked
  FROM users u
  LEFT JOIN countries c ON u.country_id = c.id;
