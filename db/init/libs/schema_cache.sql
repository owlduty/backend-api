drop schema if exists schema_cache cascade;
create schema schema_cache;
set search_path to schema_cache, public;

CREATE OR REPLACE FUNCTION schema_cache.refresh() RETURNS event_trigger
  LANGUAGE plpgsql
  AS $$
BEGIN
  NOTIFY pgrst, 'reload schema';
END;
$$;

-- This event trigger will fire after every ddl_command_end event
CREATE EVENT TRIGGER refresh
  ON ddl_command_end
  EXECUTE PROCEDURE schema_cache.refresh();
