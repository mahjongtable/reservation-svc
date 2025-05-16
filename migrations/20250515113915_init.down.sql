-- Add down migration script here
DROP SCHEMA reservation CASCADE;
DROP EXTENSION btree_gist;
DROP EXTENSION IF EXISTS "uuid-ossp";