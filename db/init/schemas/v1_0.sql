-- Exposed schema to provide API accesses to users
drop schema if exists v1_0 cascade;
create schema v1_0;
set search_path = v1_0, public;

-- this role will be used as the owner of the views in the api schemas
-- (e.g v1_0) it is needed for the definition of the RLS policies.
drop role if exists api;
create role api;

-- this is a workaround for manage databases where the master user
-- does not have SUPERUSER privileges
grant api to current_user;

-- v1_0 privileges
grant usage on schema v1_0 to anonymous, webuser;
