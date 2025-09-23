CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT,
    full_name TEXT NOT NULL,
    image_url TEXT NOT NULL,
    stripe_customer_id TEXT UNIQUE DEFAULT NULL,
    country TEXT DEFAULT NULL,
    country_code TEXT DEFAULT NULL,
    region TEXT DEFAULT NULL,
    verified BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create index idx_users_email on users(email);
