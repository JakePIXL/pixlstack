-- Add migration script here
CREATE TABLE categories (
    id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);