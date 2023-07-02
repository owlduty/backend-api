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

create or replace function v1_0.signup(name text, email text, password text) returns json as $$
declare
    usr auth.users;
begin
    insert into auth.users as u (name, email, pass) values ($1, $2, $3)
    returning * into usr;

    return auth.payload(usr);
end
$$ security definer language plpgsql;

revoke all privileges on function v1_0.signup(text, text, text) from public;
grant execute on function v1_0.signup(text,text,text) to anonymous;


create or replace function v1_0.signin(email text, password text) returns json as $$
declare
    usr auth.users;
begin
	select * from auth.users as u
    where u.email = $1 and u.pass = public.crypt($2, u.pass)
   	INTO usr;

    if usr is NULL then
        raise exception 'invalid credentials';
    else
        return auth.payload(usr);
    end if;
end
$$ stable security definer language plpgsql;
revoke all privileges on function v1_0.signin(text, text) from public;
grant execute on function v1_0.signin(text,text) to anonymous, webuser;

set search_path to public;
