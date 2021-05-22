## Install pre-commit hooks
```
pip install pre-commit
pre-commit install
```

## Execute the schema.sql file to create the database
```
psql -U postgres -W -h DATABASE_HOST -p DATABASE_PORT
> CREATE DATABASE sct;
psql -U postgres -W -h DATABASE_HOST -p DATABASE_PORT -f schema.sql sct
```
