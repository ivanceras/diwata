-- Table: film_actor_awards

-- DROP TABLE film_actor_awards;

CREATE TABLE film_actor_awards
(
actor_id smallint NOT NULL,
film_id smallint NOT NULL,
award character varying NOT NULL,
CONSTRAINT film_actor_awards_pkey PRIMARY KEY (actor_id, film_id, award),
CONSTRAINT film_actor_awards_actor_id_film_id_fkey FOREIGN KEY (actor_id, film_id)
  REFERENCES film_actor (actor_id, film_id) MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
      OIDS=FALSE
);
ALTER TABLE film_actor_awards
  OWNER TO postgres;

