-- CREATE TYPE RoomState AS ENUM ('not_started', 'started', 'ended');

CREATE TABLE rooms (
    id UUID PRIMARY KEY NOT NULL,
    -- state RoomState NOT NULL DEFAULT 'not_started',
    state VARCHAR(16) NOT NULL DEFAULT 'not_started',
    timestamp TIMESTAMP
);

CREATE TABLE players (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(16) NOT NULL,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE
);

