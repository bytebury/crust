CREATE TABLE blogs (
    id INTEGER PRIMARY KEY NOT NULL,
    slug TEXT NOT NULL GENERATED ALWAYS AS (LOWER(REPLACE(TRIM(title), ' ', '-'))) VIRTUAL UNIQUE,
    title TEXT NOT NULL CHECK(LENGTH(TRIM(title)) > 0),
    content TEXT NOT NULL CHECK(LENGTH(TRIM(content)) > 0),
    image_url TEXT NOT NULL CHECK(LENGTH(TRIM(image_url)) > 0),
    author_id INTEGER REFERENCES users(id) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX idx_blogs_slug ON blogs(slug);

CREATE VIEW blog_posts AS
SELECT b.id,
	   b.author_id,
       b.slug,
       b.title,
       b.content,
       b.image_url,
       b.created_at,
       b.updated_at,
       u.full_name as "author_name!",
       u.image_url as "author_image_url!"
FROM blogs b
LEFT JOIN users u ON b.author_id = u.id;
