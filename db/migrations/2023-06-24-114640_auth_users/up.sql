create schema auth;
set search_path to auth;

create type user_role as enum ('webuser');

-- A team allows us to get the number of seats, to attach information
-- such as credits, subscription. A team doesn't have necessarily a
-- name. In case of only one user the team is implicit.
create table auth.teams (
  id uuid primary key default gen_random_uuid(),
  name text
);

create table auth.users (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  email public.citext not null unique,
  pass text not null,
  "role" user_role not null default 'webuser',
  team_id uuid references auth.teams(id) NOT NULL

  check (length(name) > 2),
  -- email REGEX (RFC 5322)
  check (email ~* $RegexTag$^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$$RegexTag$)
);

-- Retrieve user role
create or replace function auth.user_role(email text, pass text) returns name as $$
begin
  return (
    select role from auth.users
    where users.email = email
    and users.pass = crypt(pass, users.pass)
  );
end;
$$ language plpgsql;

-- Encrypt a password in case of a new user or if
-- the password is different (updated).
create or replace function auth.encrypt_pass() returns trigger as $$
begin
  if tg_op = 'INSERT' or new.pass <> old.pass then
    new.pass = crypt(new.pass, gen_salt('bf'));
  end if;
  return new;
end
$$ language plpgsql;

create trigger user_encrypt_pass_trigger
  before insert or update on auth.users
  for each row
  execute procedure auth.encrypt_pass();

set search_path to public;
