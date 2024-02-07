CREATE TABLE users (
   id serial PRIMARY KEY NOT NULL,
   admin boolean NOT NULL DEFAULT FALSE,
   username text NOT NULL,
   email text NOT NULL,
   password text NOT NULL,
   providers text[] NOT NULL,
   services text[] NOT NULL,
   tokens text[] NOT NULL,
   login_session text NOT NULL DEFAULT ''
);