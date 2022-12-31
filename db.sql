CREATE TABLE IF NOT EXISTS player
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    count INTEGER DEFAULT 0
);