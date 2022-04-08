-- CREATE TYPE GameState AS ENUM ('not_started', 'started', 'ended');

CREATE TABLE games (
    id UUID PRIMARY KEY NOT NULL,
    -- state GameState NOT NULL DEFAULT 'not_started',
    state VARCHAR(16) NOT NULL DEFAULT 'not_started',
    timestamp TIMESTAMP
);

CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(16) NOT NULL,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
);

