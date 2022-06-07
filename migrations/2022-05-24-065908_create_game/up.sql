-- PostgresSQL:
--  Language: postgresql
--  Path: migrations/2022-05-24-065908_create_game/up.sql
--  Path: migrations/2022-05-24-065908_create_game/down.sql
CREATE TABLE "games" (
  "id" serial NOT NULL,
  ids_players integer[] NOT NULL,
  "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  CONSTRAINT "game_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "rounds" (
  "id" serial NOT NULL,
  "game_id" integer NOT NULL,
  "data" text NOT NULL,
  "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  CONSTRAINT "round_pkey" PRIMARY KEY ("id"),
  CONSTRAINT "round_game_id_fkey" FOREIGN KEY ("game_id")
      REFERENCES "games" ("id") MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)