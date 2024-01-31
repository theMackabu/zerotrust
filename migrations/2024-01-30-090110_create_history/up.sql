CREATE TABLE login_history (
   id INTEGER PRIMARY KEY NOT NULL,
   user_id INTEGER NOT NULL REFERENCES users(id),
   login_timestamp TIMESTAMP NOT NULL
);