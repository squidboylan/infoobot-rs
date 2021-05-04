-- Add migration script here
CREATE TABLE IF NOT EXISTS karma
(
    name        VARCHAR PRIMARY KEY NOT NULL,
    karma       INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS quotes
(
    key        VARCHAR PRIMARY KEY NOT NULL,
    quote      VARCHAR NOT NULL
);
