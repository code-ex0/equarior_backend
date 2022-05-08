-- PostgresSQL:
--  Language: postgresql
--  Path: migrations/2022-05-06-064043_create_users/up.sql
--  Path: migrations/2022-05-06-064043_create_users/down.sql
CREATE TABLE "users" (
                        "id" SERIAL PRIMARY KEY,
                        "username" VARCHAR(255) NOT NULL,
                        "email" VARCHAR(255) UNIQUE NOT NULL,
                        "password" VARCHAR(255) NOT NULL,
                        "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
                        "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
