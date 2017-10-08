
mysql -u root -p < ../sakila/mysql-sakila-db/sakila-schema.sql 
mysql -u root -p < ../sakila/mysql-sakila-db/sakila-data.sql

psql -U postgres -h localhost -d sakila -f ../sakila/postgres-sakila-db/postgres-sakila-schema.sql 
psql -U postgres -h localhost -d sakila -f ../sakila/postgres-sakila-db/postgres-sakila-data.sql

sqlite3 /tmp/sakila.db < ../sakila/sqlite-sakila-db/sqlite-sakila-schema.sql
sqlite3 /tmp/sakila.db < ../sakila/sqlite-sakila-db/sqlite-sakila-insert-data.sql

