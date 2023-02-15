-- Add migration script here
CREATE TABLE users (
  id uuid NOT NULL,
  role text,
  is_verified boolean DEFAULT false,
  is_active boolean DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  PRIMARY KEY (id)
);
