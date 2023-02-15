-- Add migration script here
CREATE TABLE contact_requests (
  id UUID NOT NULL,
  name VARCHAR(255) NOT NULL,
  company_name VARCHAR(255) DEFAULT NULL,
  email VARCHAR(255) NOT NULL,
  urgency VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at timestamptz DEFAULT NOW(),
  PRIMARY KEY (id)
);