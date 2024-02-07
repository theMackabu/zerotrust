CREATE TABLE login_history (
   id serial PRIMARY KEY NOT NULL,
   user_id integer NOT NULL REFERENCES users(id),
   login_timestamp TIMESTAMP NOT NULL
);