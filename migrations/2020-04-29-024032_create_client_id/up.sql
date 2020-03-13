-- Your SQL goes here
CREATE TABLE "client_credential" (
    id VARCHAR NOT NULL PRIMARY KEY,
    secret VARCHAR NOT NULL,
    redirect_uri VARCHAR NOT NULL
)
