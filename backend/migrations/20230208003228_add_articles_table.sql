-- Add migration script here
CREATE TABLE articles (
    id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    excerpt TEXT NOT NULL,
    content TEXT NOT NULL,
    category_id UUID DEFAULT NULL,
    is_published BOOLEAN DEFAULT FALSE,
    created_at timestamptz DEFAULT NOW(),
    updated_at timestamptz DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);