-- CREATE TYPE GameState AS ENUM ('not_started', 'started', 'ended');

CREATE TABLE games (
    id UUID PRIMARY KEY NOT NULL,
    -- state GameState NOT NULL DEFAULT 'not_started',
    state VARCHAR(16) NOT NULL DEFAULT 'not_started',
    timestamp TIMESTAMP
);
