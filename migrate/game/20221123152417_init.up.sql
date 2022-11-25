-- Add up migration script here
BEGIN;


CREATE TABLE IF NOT EXISTS public.players
(
    id bigserial NOT NULL,
    "createdAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "updatedAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "deletedAt" timestamp without time zone,
    basic jsonb,
    items jsonb,
    state jsonb,
    CONSTRAINT players_id_pk PRIMARY KEY (id)
);
END;