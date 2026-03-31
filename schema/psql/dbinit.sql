/*
 * dbinit.sql: raw SQL to initialize a database for use by Lumen
 */

CREATE DATABASE lumen;
CREATE USER lumen;
ALTER DEFAULT PRIVILEGES GRANT INSERT, SELECT, UPDATE, DELETE ON TABLES TO lumen;
ALTER USER lumen PASSWORD 'lumen';

-- switch to lumen database
\c lumen;

BEGIN;

CREATE TYPE lumen.public.auth_user_provider_kind AS ENUM (
  'local'
);

CREATE TYPE lumen.public.storage_provider_kind AS ENUM (
  'local'
);

CREATE TABLE lumen.public.auth_user (
  id UUID PRIMARY KEY,
  time_created TIMESTAMPTZ NOT NULL,
  time_modified TIMESTAMPTZ NOT NULL,
  time_deleted TIMESTAMPTZ,
  email TEXT NOT NULL UNIQUE
);

CREATE TABLE lumen.public.auth_user_provider (
  id UUID PRIMARY KEY,
  time_created TIMESTAMPTZ NOT NULL,
  time_modified TIMESTAMPTZ NOT NULL,
  time_deleted TIMESTAMPTZ,
  provider_kind lumen.public.auth_user_provider_kind NOT NULL,
  provider_id TEXT NOT NULL,
  user_id UUID NOT NULL REFERENCES lumen.public.auth_user(id) ON DELETE CASCADE,

  -- only one user with an id can be ingested per provider.
  UNIQUE (provider_kind, provider_id),

  -- a user can only have one provider of a given kind.
  UNIQUE (provider_kind, user_id)
);

CREATE TABLE lumen.public.auth_user_password (
  ID UUID PRIMARY KEY,
  time_created TIMESTAMPTZ NOT NULL,
  password_hash TEXT NOT NULL,
  user_id UUID NOT NULL REFERENCES lumen.public.auth_user(id) ON DELETE CASCADE,

  -- a user can have multiple passwords over time, but they cannot reuse them.
  UNIQUE (password_hash, user_id)
);

CREATE TABLE lumen.public.storage_object (
  id UUID PRIMARY KEY,
  time_created TIMESTAMPTZ NOT NULL,
  time_modified TIMESTAMPTZ NOT NULL,
  time_deleted TIMESTAMPTZ,
  provider_kind lumen.public.storage_provider_kind NOT NULL,
  provider_path TEXT NOT NULL,
  mime_type TEXT NOT NULL
);

COMMIT;
