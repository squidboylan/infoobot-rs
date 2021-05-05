-- Add migration script here
CREATE TABLE IF NOT EXISTS karma
(
    guild_id    INTEGER NOT NULL,
    name        VARCHAR NOT NULL,
    karma       INTEGER DEFAULT 0,
    PRIMARY KEY(guild_id, name)
);

CREATE TABLE IF NOT EXISTS quotes
(
    guild_id   INTEGER NOT NULL,
    key        VARCHAR NOT NULL,
    quote      VARCHAR NOT NULL,
    PRIMARY KEY(guild_id, key)
);
