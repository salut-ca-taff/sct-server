BEGIN;

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'maths', 'Mathematiques' );

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'francais', 'Français' );

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'histoire', 'Histoire' );

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'geo', 'Geographie' );

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'physique', 'Physique' );

INSERT INTO "subjects" ( "slug", "title" )
VALUES ( 'chimie', 'Chimie' );

INSERT INTO "chapters" ( "subject", "title" )
VALUES ( ( SELECT "id" FROM "subjects" WHERE "slug" = 'maths' ), 'Développements limité' );

INSERT INTO "chapters" ( "subject", "title" )
VALUES ( ( SELECT "id" FROM "subjects" WHERE "slug" = 'maths' ), 'Espaces vectoriels' );

INSERT INTO "chapters" ( "subject", "title" )
VALUES ( ( SELECT "id" FROM "subjects" WHERE "slug" = 'maths' ), 'Matrices' );

INSERT INTO "chapters" ( "subject", "title" )
VALUES ( ( SELECT "id" FROM "subjects" WHERE "slug" = 'maths' ), 'Groupes symétrique et déterminants' );

COMMIT;
