CREATE TABLE "users" (
  "email" varchar(100) PRIMARY KEY NOT NULL,
  "username" varchar(100) NOT NULL,
  "salt" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "school" varchar(255) NOT NULL
);

CREATE TABLE "comments" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "author" varchar(32) NOT NULL,
  "course" int NOT NULL,
  "content" varchar(1000),
  "created_at" timestamp NOT NULL DEFAULT (current_timestamp)
);

CREATE TABLE "courses" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "chapter" int NOT NULL,
  "author" varchar(32) NOT NULL,
  "title" varchar(255) NOT NULL,
  "small" boolean NOT NULL DEFAULT FALSE,
  "level" varchar(32) NOT NULL,
  "content" varchar(1000),
  "view_count" int NOT NULL DEFAULT 0,
  "created_at" timestamp NOT NULL DEFAULT (current_timestamp)
);

CREATE TABLE "annals" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "chapter" int NOT NULL,
  "author" varchar(32) NOT NULL,
  "title" varchar(255) NOT NULL,
  "small" boolean NOT NULL DEFAULT FALSE,
  "level" varchar(32) NOT NULL,
  "subject" varchar(1000),
  "corrected" varchar(1000),
  "view_count" int NOT NULL DEFAULT 0,
  "created_at" timestamp NOT NULL DEFAULT (current_timestamp)
);

CREATE TABLE "stars" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "value" int NOT NULL,
  "user" varchar(32) NOT NULL,
  "course" int NOT NULL
);

CREATE TABLE "chapters" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "subject" int NOT NULL,
  "title" varchar(255) NOT NULL,
  "description" varchar(255)
);

CREATE TABLE "subjects" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "slug" varchar(255) NOT NULL UNIQUE,
  "title" varchar(255) NOT NULL,
  "color" varchar(32)
);

ALTER TABLE "comments" ADD FOREIGN KEY ("author") REFERENCES "users" ("email");

ALTER TABLE "comments" ADD FOREIGN KEY ("course") REFERENCES "courses" ("id");

ALTER TABLE "courses" ADD FOREIGN KEY ("chapter") REFERENCES "chapters" ("id");

ALTER TABLE "courses" ADD FOREIGN KEY ("author") REFERENCES "users" ("email");

ALTER TABLE "annals" ADD FOREIGN KEY ("chapter") REFERENCES "chapters" ("id");

ALTER TABLE "annals" ADD FOREIGN KEY ("author") REFERENCES "users" ("email");

ALTER TABLE "stars" ADD FOREIGN KEY ("user") REFERENCES "users" ("email");

ALTER TABLE "stars" ADD FOREIGN KEY ("course") REFERENCES "courses" ("id");

ALTER TABLE "chapters" ADD FOREIGN KEY ("subject") REFERENCES "subjects" ("id");
