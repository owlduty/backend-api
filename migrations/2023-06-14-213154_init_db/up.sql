create schema api;

create type api.test_status as ENUM ('passed', 'failed', 'alert', 'paused', 'init');
create type api.test_priority as ENUM ('priority 1', 'priority 2', 'priority 3', 'priority 4', 'priority 5');
create type api.test_type as ENUM ('http', 'websocket', 'grpc', 'ssl', 'dns', 'tcp', 'udp', 'icmp');

create table api.tests (
  id uuid primary key,
  status api.test_status,
  priority api.test_priority,
  name text,
  type api.test_type,
  domain text,
  uptime double precision,
  tags text[]
);
