set search_path to v1_0, public;

create or replace function v1_0.signup(name text, email text, password text) returns json as $$
declare
    usr record;
begin
    insert into auth.users as u
    (name, email, pass) values ($1, $2, $3)
    returning *
   	into usr;

    return json_build_object(
        'profile', json_build_object(
            'id', usr.id,
            'name', usr.name,
            'email', usr.email
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

revoke all privileges on function v1_0.signup(text, text, text) from public;
grant execute on function v1_0.signup(text,text,text) to anonymous;

set search_path to public;
