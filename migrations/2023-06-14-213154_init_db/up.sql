-- The exposed API via PostgREST
create schema v1_0;

-- Private schemas
create schema synthetic;

-- Roles
create role anonymous nologin;

-- Types
create type synthetic.run_status as ENUM ('passed', 'failed', 'alert', 'paused', 'init');
create type synthetic.run_type as ENUM ('scheduled', 'triggered');
create type synthetic.test_priority as ENUM ('priority 1', 'priority 2', 'priority 3', 'priority 4', 'priority 5');
create type synthetic.test_type as ENUM ('http', 'websocket', 'grpc', 'ssl', 'dns', 'tcp', 'udp', 'icmp');

create table synthetic.tests (
  id uuid primary key,
  name text NOT NULL,
  type synthetic.test_type NOT NULL,
  priority synthetic.test_priority,
  uptime double precision NOT NULL CHECK (uptime >= 0) DEFAULT 0,
  tags text[]
);

create table synthetic.steps (
  id uuid primary key,
  name text NOT NULL,
  position smallint NOT NULL CHECK (position >= 0),
  assertions json NOT NULL,
  request json NOT NULL,
  test uuid references synthetic.tests(id) NOT NULL,
  UNIQUE(test, name, position)
);

create table synthetic.runs (
  id uuid primary key,
  status synthetic.run_status NOT NULL DEFAULT 'init',
  type synthetic.run_type NOT NULL,
  scheduling varchar(7), --cron expression with seconds
  response json
);
