CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT,
    full_name TEXT NOT NULL,
    image_url TEXT NOT NULL,
    stripe_customer_id TEXT UNIQUE DEFAULT NULL,
    country_id INTEGER REFERENCES countries(id) DEFAULT NULL,
    region TEXT DEFAULT NULL,
    verified BOOLEAN NOT NULL DEFAULT 0,
    locked BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create index idx_users_email on users(email);

-- view that we will use when retrieving users.
CREATE VIEW v_users AS
SELECT u.*,
       c.name as "country_name",
       c.code as "country_code",
       c.locked as "country_locked"
  FROM users u
  LEFT JOIN countries c ON u.country_id = c.id;

