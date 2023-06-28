-- This init configuration file is executed when the docker container starts. We
-- mount it from the docker-compose.yml file.
-- The entrypoint executes /docker-entrypoint-initdb.d/*.{sh,sql} files.

\set QUIET on
\set ON_ERROR_STOP on
set client_min_messages to warning;

-- load some variables from the env
\set anonymous `echo $DB_ANON_ROLE`
\set authenticator `echo $DB_USER`
\set authenticator_pass `echo $DB_USER_PASS`
\set jwt_secret `echo $JWT_SECRET`
\set jwt_lifetime `echo $JWT_LIFETIME`
begin;

\echo # ---------- Database initialization: starting ----------

\echo # Create extensions: pgcrypto, citext
create extension if not exists pgcrypto;
create extension if not exists citext;

\echo # Loading libraries

-- Functions for JWT token generation in the database context
\ir libs/pgjwt.sql

-- Function for storing settings in a table with a secure access
\ir libs/settings.sql

-- Http request/response functions to manage cookies
\ir libs/request.sql
\ir libs/response.sql

\echo # Loading roles

-- Create API roles
\ir auth/roles.sql

-- Save app settings
\echo # Saving app settings
select settings.set('jwt_secret', :'jwt_secret');
select settings.set('jwt_lifetime', :'jwt_lifetime');

-- Exposed api schema
\echo # Create exposed api schemas
\ir schemas/v1_0.sql

commit;
\echo # ---------- Database initialization: done ----------
