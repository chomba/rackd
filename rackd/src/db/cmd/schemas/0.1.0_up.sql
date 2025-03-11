CREATE TABLE IF NOT EXISTS key_value (
    key     TEXT    PRIMARY KEY,
    value   TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS event (
    seq         INTEGER     PRIMARY KEY,
    id          TEXT        NOT NULL UNIQUE,
    stream_id   TEXT        NOT NULL,
    version     INTEGER     NOT NULL,
    data        TEXT        NOT NULL
);

CREATE TABLE IF NOT EXISTS wan (
    seq             INTEGER     PRIMARY KEY,
    version         INTEGER     NOT NULL,
    id              TEXT        NOT NULL UNIQUE,
    rack            TEXT        NOT NULL,
    trunk           INTEGER     NOT NULL,
    vlan            INTEGER     NOT NULL,
    conn            TEXT        NOT NULL,
    name            TEXT        NOT NULL,
    mac             TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS lan4 (
    seq             INTEGER     PRIMARY KEY,
    version         INTEGER     NOT NULL,
    id              TEXT        NOT NULL UNIQUE,
    name            TEXT        NOT NULL,
    prefix          TEXT        NOT NULL, 
    iprefix         TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);


CREATE TABLE IF NOT EXISTS lan4_descendant (
    id                  TEXT    NOT NULL,
    descendant_id       TEXT    NOT NULL,
    UNIQUE(id, descendant_id) ON CONFLICT REPLACE
);

CREATE TABLE IF NOT EXISTS lan6 (
    seq             INTEGER     PRIMARY KEY,
    version         INTEGER     NOT NULL,
    id              TEXT        NOT NULL UNIQUE,
    name            TEXT        NOT NULL,
    prefix          TEXT        NOT NULL, 
    iprefix         TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);


CREATE TABLE IF NOT EXISTS lan6_descendant (
    id                  TEXT    NOT NULL,
    descendant_id       TEXT    NOT NULL,
    UNIQUE(id, descendant_id) ON CONFLICT REPLACE
);

CREATE TABLE IF NOT EXISTS wan4 (
    seq             INTEGER     PRIMARY KEY,
    id              TEXT        NOT NULL UNIQUE,
    name            TEXT        NOT NULL,
    prefix          TEXT        NOT NULL,
    iprefix         TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS wan6 (
    seq             INTEGER     PRIMARY KEY,
    id              TEXT        NOT NULL UNIQUE,
    name            TEXT        NOT NULL,
    prefix          TEXT        NOT NULL,
    iprefix         TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS snat6 (
    seq                     INTEGER     PRIMARY KEY,
    version                 INTEGER     NOT NULL,
    id                      TEXT        NOT NULL UNIQUE,
    prefix                  TEXT        NOT NULL,
    iprefix                 TEXT        NOT NULL,
    targets                 TEXT        NOT NULL,
    mode                    TEXT        NOT NULL,
    status                  TEXT        NOT NULL
);       

CREATE TABLE IF NOT EXISTS snat6_target (
    seq             INTEGER     PRIMARY KEY,
    id              TEXT        NOT NULL UNIQUE,
    prefix          TEXT        NOT NULL,
    ipv6_prefix     TEXT        NOT NULL,
    snat_id         TEXT        NOT NULL
);
