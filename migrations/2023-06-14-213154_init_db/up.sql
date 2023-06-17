-- The exposed API via PostgREST
CREATE SCHEMA v1_0;

-- Private schemas
CREATE SCHEMA synthetic;

-- Roles
CREATE ROLE anonymous nologin;

-- Types
CREATE TYPE synthetic.run_status AS ENUM ('passed', 'failed', 'alert', 'paused', 'init');
CREATE TYPE synthetic.run_type AS ENUM ('scheduled', 'triggered');
CREATE TYPE synthetic.test_priority AS ENUM ('priority 1', 'priority 2', 'priority 3', 'priority 4', 'priority 5');
CREATE TYPE synthetic.test_type AS ENUM ('http', 'websocket', 'grpc', 'ssl', 'dns', 'tcp', 'udp', 'icmp');

CREATE TABLE synthetic.tests (
  id uuid PRIMARY KEY,
  name text NOT NULL,
  type synthetic.test_type NOT NULL,
  priority synthetic.test_priority,
  uptime double precision NOT NULL CHECK (uptime >= 0) DEFAULT 0,
  tags text[],
  locations text[] NOT NULL DEFAULT ARRAY['paris']
);

CREATE TABLE synthetic.steps (
  id uuid PRIMARY KEY,
  name text NOT NULL,
  position smallint NOT NULL CHECK (position >= 0),
  assertions json NOT NULL,
  request json NOT NULL,
  response json,
  test uuid references synthetic.tests(id) NOT NULL,
  UNIQUE(test, name, position)
);

CREATE TABLE synthetic.runs (
  id uuid PRIMARY KEY,
  status synthetic.run_status NOT NULL DEFAULT 'init',
  type synthetic.run_type NOT NULL,
  scheduling varchar(7), --cron expression with seconds
  test uuid references synthetic.tests(id) NOT NULL
);

-- public schema views

CREATE view v1_0.synthetics AS
  SELECT * FROM synthetic.tests;
