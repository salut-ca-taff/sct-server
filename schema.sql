CREATE TABLE "users" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "username" varchar(32) NOT NULL,
  "password" varchar(255) NOT NULL,
  "email" varchar(100) NOT NULL,
  "display_name" varchar(32) NOT NULL,
  "school" varchar(255) NOT NULL
);

CREATE TABLE "comments" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "author" int NOT NULL,
  "content" varchar(1000),
  "attachments" varchar(65535),
  "created_at" timestamp NOT NULL DEFAULT current_timestamp
);

CREATE TABLE "resources" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "chapter" int NOT NULL,
  "author" int NOT NULL,
  "title" varchar(255) NOT NULL,
  "content" varchar(1000),
  "attachments" varchar(65535),
  "view_count" int NOT NULL DEFAULT 0,
  "created_at" timestamp NOT NULL DEFAULT current_timestamp
);

CREATE TABLE "stars" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "value" int NOT NULL,
  "user" int NOT NULL,
  "resource" int NOT NULL
);

CREATE TABLE "chapters" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "subject" int NOT NULL,
  "title" varchar(255) NOT NULL,
  "description" varchar(255)
);

CREATE TABLE "subjects" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "title" varchar(255) NOT NULL
);

ALTER TABLE "comments" ADD FOREIGN KEY ("author") REFERENCES "users" ("id");

ALTER TABLE "resources" ADD FOREIGN KEY ("chapter") REFERENCES "chapters" ("id");

ALTER TABLE "resources" ADD FOREIGN KEY ("author") REFERENCES "users" ("id");

ALTER TABLE "stars" ADD FOREIGN KEY ("user") REFERENCES "users" ("id");

ALTER TABLE "stars" ADD FOREIGN KEY ("resource") REFERENCES "resources" ("id");

ALTER TABLE "chapters" ADD FOREIGN KEY ("subject") REFERENCES "subjects" ("id");
