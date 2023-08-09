set search_path to v1_0, public;

create or replace function auth.payload(usr auth.users) returns json as $$
begin
    return json_build_object(
        'profile', json_build_object(
            'id', usr.id
        ),
        'token', pgjwt.sign(
            json_build_object(
                'role', usr.role,
                'user_id', usr.id,
                'exp', extract(epoch from now())::integer + settings.get('jwt_lifetime')::int -- token expires in 1 hour
            ),
            settings.get('jwt_secret')
        )
    );
end
$$ security definer language plpgsql;
revoke all privileges on function auth.payload(auth.users) from public;

create or replace function v1_0.signup(name text, email text, password text, team_id uuid default null) returns json as $$
declare
    usr auth.users;
    team auth.teams;
begin
    -- Create the team if the user doesn't have one or retrieve it
    if team_id is NULL then
        insert into auth.teams as t default values
        returning * into team;
    else
        select * from auth.teams as t
        where t.id = team_id
   	    into team;
    end if;

    -- check the team is correctly set
    if team is NULL then
        raise exception 'invalid team';
    end if;

    insert into auth.users as u (name, email, pass, team_id) values ($1, $2, $3, team.id)
    returning * into usr;

    return auth.payload(usr);
end
$$ security definer language plpgsql;

-- create or replace function v1_0.signup(name text, email text, password text) returns json as $$
-- begin
--     return auth.signup(name, email, password);
-- end
-- $$ security definer language plpgsql;

-- create or replace function v1_0.signup(name text, email text, password text, team_id uuid) returns json as $$
-- begin
--     return auth.signup(name, email, password, team_id);
-- end
-- $$ security definer language plpgsql;

-- revoke all privileges on function v1_0.signup(text, text, text) from public;
revoke all privileges on function v1_0.signup(text, text, text, uuid) from public;
-- grant execute on function v1_0.signup(text, text, text) to anonymous;
grant execute on function v1_0.signup(text, text, text, uuid) to anonymous;

create or replace function v1_0.signin(email text, password text) returns json as $$
declare
    usr auth.users;
begin

	select * from auth.users as u
    where u.email = $1 and u.pass = public.crypt($2, u.pass)
   	into usr;

    if usr is NULL then
        raise exception 'invalid credentials';
    else
        return auth.payload(usr);
    end if;
end
$$ stable security definer language plpgsql;
revoke all privileges on function v1_0.signin(text, text) from public;
grant execute on function v1_0.signin(text, text) to anonymous, webuser;

create or replace function v1_0.refresh_token() returns text as $$
declare
    usr auth.users;
    token text;
begin

    select * from data."user" as u
    where id = request.user_id()
    into usr;

    if usr is null then
        raise exception 'user not found';
    else
        token := pgjwt.sign(
            json_build_object(
                'role', usr.role,
                'user_id', usr.id,
                'exp', extract(epoch from now())::integer + settings.get('jwt_lifetime')::int
            ),
            settings.get('jwt_secret')
        );
        return token;
    end if;
end
$$ stable security definer language plpgsql;

revoke all privileges on function v1_0.refresh_token() from public;
grant execute on function v1_0.refresh_token() to webuser;

set search_path to public;
