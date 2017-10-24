
psql -U postgres -h localhost -d sakila -c "DROP schema public CASCADE; CREATE schema public;"
psql -U postgres -h localhost -d sakila -f ../sakila/postgres-sakila-db/postgres-sakila-schema.sql 
psql -U postgres -h localhost -d sakila -f ../sakila/postgres-sakila-db/postgres-sakila-data.sql
psql -U postgres -h localhost -d sakila -f ./composite_foreign_key.sql
psql -U postgres -h localhost -d sakila -f ./add_users.sql
