-- Add down migration script here
BEGIN;
    DROP TABLE IF EXISTS public.players;
END;