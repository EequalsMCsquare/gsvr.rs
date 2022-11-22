BEGIN;


-- create table 'accounts'
-- this table is used to store player's account but not the actual game role data
CREATE TABLE IF NOT EXISTS public.accounts
(
    id bigserial NOT NULL,
    username character varying(64) NOT NULL,
    password character varying(256) NOT NULL,
    email character varying(64),
    phone character varying(32),
    "createdAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "updatedAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "deletedAt" timestamp without time zone,
    "lastLogin" timestamp without time zone,
    CONSTRAINT accounts_pk PRIMARY KEY (id),
    CONSTRAINT accounts_username_key UNIQUE (username),
    CONSTRAINT accounts_email_key UNIQUE (email),
    CONSTRAINT accounts_phone_key UNIQUE (phone)
);

-- create table 'players'
-- each account holds N players
CREATE TABLE IF NOT EXISTS public.players
(
    "accountId" bigserial NOT NULL,
    id integer NOT NULL,
    name character varying(16) NOT NULL,
    "createdAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "updatedAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "deletedAt" timestamp without time zone,
    "lastLogin" timestamp without time zone,
    CONSTRAINT players_pk PRIMARY KEY ("accountId", id)
);

-- add foreign key constrain players.account_id -> accounts.id
ALTER TABLE IF EXISTS public.players
    ADD CONSTRAINT account_id_fk FOREIGN KEY ("accountId")
    REFERENCES public.accounts (id) MATCH SIMPLE
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT VALID;