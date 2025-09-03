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

CREATE TABLE IF NOT EXISTS entity (
    id      TEXT      PRIMARY KEY,
    value   TEXT      NOT NULL
);

CREATE TABLE IF NOT EXISTS network_view (
    id              TEXT        PRIMARY KEY,
    trunk_id        TEXT        NOT NULL,
    trunk_name      TEXT        NOT NULL,
    vlan            INTEGER     NOT NULL,
    name            TEXT        NOT NULL,
    kind            TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS wan_view (
    id              TEXT        PRIMARY KEY,
    rack_id         TEXT        NOT NULL,
    rack_asn        INTEGER     NOT NULL,
    trunk_id        TEXT        NOT NULL,
    trunk_name      TEXT        NOT NULL,
    vlan            INTEGER     NOT NULL,
    name            TEXT        NOT NULL,
    mode            TEXT        NOT NULL,
    deleted         INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS trunk_view (
    id              TEXT        PRIMARY KEY,
    name            TEXT        NOT NULL UNIQUE,
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
