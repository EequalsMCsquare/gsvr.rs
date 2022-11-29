-- Add up migration script here
BEGIN;


CREATE TABLE IF NOT EXISTS public.players
(
    id bigserial NOT NULL,
    basic jsonb,
    "createdAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "updatedAt" timestamp without time zone NOT NULL DEFAULT (now() at time zone 'utc'),
    "deletedAt" timestamp without time zone,
    CONSTRAINT players_id_pk PRIMARY KEY (id)
);
END;