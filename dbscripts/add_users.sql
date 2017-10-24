
-- Table: users

-- DROP TABLE users;

CREATE TABLE users
(
  user_id serial NOT NULL,
  username character varying,
  password character varying,
  photo bytea,
  email character varying,
  CONSTRAINT users_pkey PRIMARY KEY (user_id),
  CONSTRAINT user_email_uniq UNIQUE (email),
  CONSTRAINT users_username_uniq UNIQUE (username)
)
WITH (
  OIDS=FALSE
);
ALTER TABLE users
  OWNER TO postgres;
