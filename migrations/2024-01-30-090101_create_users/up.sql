CREATE TABLE users (
   id INTEGER PRIMARY KEY NOT NULL,
   admin INTEGER NOT NULL DEFAULT FALSE,
   username VARCHAR NOT NULL,
   email VARCHAR NOT NULL,
   password VARCHAR NOT NULL,
   providers VARCHAR NOT NULL,
   services VARCHAR NOT NULL,
   login_session VARCHAR NOT NULL DEFAULT ''
);