-- PG settings can be read by anyone having access to the database. For better
-- security we provide a way to save secrets in a table and secure its accesses
-- with security definer.

drop schema if exists settings cascade;
create schema settings;
set search_path to settings, public;

create table settings.secrets (
	key    text primary key,
	value  text not null
);


create or replace function settings.get(text) returns text as $$
    select value from settings.secrets where key = $1
$$ security definer stable language sql;

create or replace function settings.set(text, text) returns void as $$
	insert into settings.secrets (key, value)
	values ($1, $2)
	on conflict (key) do update
	set value = $2;
$$ security definer language sql;

set search_path to public;
