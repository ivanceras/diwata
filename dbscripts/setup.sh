psql -U postgres -h localhost -c 'DROP DATABASE sakila;'
psql -U postgres -h localhost -c 'CREATE DATABASE sakila;'
psql -U postgres -h localhost -d sakila -f ./sakila/postgres-sakila-db/postgres-sakila-schema.sql
psql -U postgres -h localhost -d sakila -f ./sakila/postgres-sakila-db/postgres-sakila-data.sql
psql -U postgres -h localhost -d sakila -f ./dbscripts/add_users.sql
psql -U postgres -h localhost -d sakila -f ./dbscripts/composite_foreign_key.sql
psql -U postgres -h localhost -c "ALTER USER postgres WITH PASSWORD 'p0stgr3s';"

