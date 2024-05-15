-- Add migration script here
-- Create subscription Table
CREATE TABLE subscriptions (
  id uuid NOT null,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz NOT NULL
);
