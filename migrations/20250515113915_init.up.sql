-- Add up migration script here
CREATE SCHEMA reservation;
CREATE EXTENSION btree_gist;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";