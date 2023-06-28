-- This file contains the definition of the applications specific roles.
-- They should not be made owners of database entities (tables/views/...).

-- Role used by postgrest to connect to the database. It can only switch
-- to new roles.
drop role if exists :authenticator;
create role :"authenticator" with login password :'authenticator_pass';

-- Application level role for unauthenticated requests.
drop role if exists :"anonymous";
create role :"anonymous";
grant :"anonymous" to :"authenticator";

-- role for the main application user accessing the api
drop role if exists webuser;
create role webuser;
grant webuser to :"authenticator";
