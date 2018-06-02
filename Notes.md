## Postgresql notes:


##  Getting user previlege per table
https://stackoverflow.com/questions/946804/find-out-if-user-got-permission-to-select-update-a-table-function-in-pos
      SQL:
select usename, nspname , relname,
       case relkind when 'r' then 'TABLE' when 'v' then 'VIEW' end as relation_type,
       priv
from pg_class join pg_namespace on pg_namespace.oid = pg_class.relnamespace,
     pg_user,
     (values('SELECT', 1),('INSERT', 2),('UPDATE', 3),('DELETE', 4)) privs(priv, privorder)
where relkind in ('r', 'v')
      and has_table_privilege(pg_user.usesysid, pg_class.oid, priv)
      and not (nspname ~ '^pg_' or nspname = 'information_schema')
order by 2, 1, 3, privorder;


privileges and sytem functions
https://www.postgresql.org/docs/9.6/static/functions-info.html

### Grant select previlege of user to all tables in a schema
GRANT SELECT ON ALL TABLES IN SCHEMA public TO user;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO user;

## revoking a select previlege of a user to a table
REVOKE SELECT ON TABLE public.film FROM lee;

##
-- granting privelege to a table implicitly grants privilege to all the columns
-- of that table.
-- threfore revoking on a specific table column does not work.
REVOKE SELECT (rental_rate) ON TABLE public.film FROM lee; -- this doesn't work. since it is overriden by the table grant

## REVOKE first to the table then grant permission to each specific column
REVOKE SELECT ON TABLE public.film FROM lee;
GRANT SELECT (rental_rate) ON TABLE public.film TO lee;


## More privileges example
https://stackoverflow.com/questions/48108860/postgresql-revoke-privileges-from-column
You could give privileges for every column. Assuming you have a table like the following:

CREATE TABLE product (
    id serial primary key,
    mytext text
);

You can grant privileges to editor_user like that:

GRANT SELECT(id), INSERT(id) ON product TO editor_user;
GRANT SELECT(mytext), UPDATE(mytext), INSERT(mytext), REFERENCES(mytext) ON product TO editor_user;

## Checking for privileges

select has_table_privilege('lee','public.actor', 'select')
select has_any_column_privilege('lee', 'public.film', 'select')
select has_column_privilege('lee', 'public.film', 'rental_rate', 'select')
select has_column_privilege('lee', 'public.film', 'title', 'select')


## Database roles:
 - read_only can read only on tables
 - app_user normal access: insert, update,
 - app_admin: insert, update, delete
 - database_admin: inser, update, delete, create, drop, truncate

