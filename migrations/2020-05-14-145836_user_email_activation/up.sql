-- Your SQL goes here
ALTER TABLE "user"
ADD COLUMN email VARCHAR UNIQUE NOT NULL default '',
ADD COLUMN is_activated BOOLEAN NOT NULL DEFAULT false;

UPDATE "user" SET is_activated=true;
