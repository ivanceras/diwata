use error::DbError;
use dao::{FromDao, ToColumns, ToDao, ToTable};
use dao::{ToValue, Value};
use platform::DBPlatform;

pub struct EntityManager(pub DBPlatform);


impl EntityManager {
    /// get all the records of this table
    pub fn get_all<T>(&self) -> Result<Vec<T>, DbError>
    where
        T: ToTable + FromDao,
    {
        let table = T::to_table();
        let sql = format!("SELECT * FROM {}", table.name());
        let rows = self.0.execute_sql_with_return(&sql, &[])?;
        let mut entities = vec![];
        for dao in rows.iter() {
            let entity = T::from_dao(&dao);
            entities.push(entity)
        }
        Ok(entities)
    }

    /// insert to table the values of this struct
    pub fn insert<T, R>(&self, entities: &[T]) -> Result<Vec<R>, DbError>
    where
        T: ToTable + ToColumns + ToDao,
        R: FromDao + ToColumns,
    {
        let table = T::to_table();
        let columns = T::to_columns();
        let columns_len = columns.len();
        let mut sql = String::new();
        sql += &format!("INSERT INTO {} ", table.name());
        sql += &format!(
            "({})\n",
            columns
                .iter()
                .map(|c| c.name.to_owned())
                .collect::<Vec<_>>()
                .join(", ")
        );
        sql += "VALUES ";
        sql += &entities
            .iter()
            .enumerate()
            .map(|(y, _)| {
                format!(
                    "\n\t({})",
                    columns
                        .iter()
                        .enumerate()
                        .map(|(x, _)| format!("${}", y * columns_len + x + 1))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let return_columns = R::to_columns();
        sql += &format!(
            "RETURNING {}",
            return_columns
                .iter()
                .map(|rc| rc.name.to_owned())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut values: Vec<Value> = Vec::with_capacity(entities.len() * columns.len());
        for entity in entities {
            let mut dao = entity.to_dao();
            for col in columns.iter() {
                let value = dao.remove(&col.name);
                match value {
                    Some(value) => values.push(value),
                    None => values.push(Value::Nil),
                }
            }
        }
        println!("sql: {}", sql);
        println!("values: {:#?}", values);
        let rows = self.0.execute_sql_with_return(&sql, &values)?;
        let mut retrieved_entities = vec![];
        for dao in rows.iter() {
            let retrieved = R::from_dao(&dao);
            retrieved_entities.push(retrieved);
        }
        Ok(retrieved_entities)
    }

    pub fn execute_sql_with_return<'a, R>(
        &self,
        sql: &str,
        params: &'a [&'a ToValue],
    ) -> Result<Vec<R>, DbError>
    where
        R: FromDao,
    {
        let values: Vec<Value> = params
            .iter()
            .map(|param| param.to_value())
            .collect::<Vec<Value>>();
        let rows = self.0.execute_sql_with_return(sql, &values)?;
        Ok(rows.iter().map(|dao| R::from_dao(&dao)).collect::<Vec<R>>())
    }
}


#[cfg(test)]
mod test {
    extern crate dao;
    use super::*;
    use dao::{FromDao, ToColumns, ToDao, ToTable};
    use pool::Pool;
    use chrono::DateTime;
    use chrono::offset::Utc;

    #[test]
    fn use_em() {
        #[derive(Debug, FromDao, ToTable)]
        struct Users {
            id: i32,
            email: String,
        }
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let mut pool = Pool::new();
        let em = pool.em(db_url).unwrap();
        let users: Result<Vec<Users>, DbError> = em.get_all();
        println!("users: {:#?}", users);
        if let Ok(users) = users {
            for user in users {
                println!("user: {:?}", user);
            }
        }
    }

    #[test]
    fn insert_some_data() {
        #[derive(Debug, FromDao, ToDao, ToColumns, ToTable)]
        struct Users {
            id: i32,
            email: String,
            username: String,
            password: String,
            created_at: DateTime<Utc>,
        }
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let mut pool = Pool::new();
        let em = pool.em(db_url).unwrap();
        let user1 = Users {
            id: 1000,
            username: "user1000".into(),
            email: "user1000@forum.org".to_string(),
            password: "user1000rocks".into(),
            created_at: Utc::now(),
        };
        let user2 = Users {
            id: 1002,
            username: "user1002".into(),
            email: "user1002@forum.org".to_string(),
            password: "user1002rocks".into(),
            created_at: Utc::now(),
        };

        let users: Result<Vec<Users>, DbError> = em.insert(&[user1, user2]);
        println!("users: {:#?}", users);
        assert!(users.is_ok());
        panic!();
    }

    #[test]
    fn insert_some_data_with_different_retrieve() {
        mod insertion {
            use super::*;
            #[derive(Debug, ToDao, ToColumns, ToTable)]
            pub struct Users {
                pub id: i32,
                pub email: String,
                pub username: String,
                pub password: String,
            }
        }

        mod retrieve {
            use super::*;
            #[derive(Debug, FromDao, ToColumns, ToTable)]
            pub struct Users {
                id: i32,
                email: String,
                username: String,
                password: String,
                created_at: DateTime<Utc>,
            }
        }


        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let mut pool = Pool::new();
        let em = pool.em(db_url).unwrap();
        let user1 = insertion::Users {
            id: 1000,
            username: "user1000".into(),
            email: "user1000@forum.org".to_string(),
            password: "user1000rocks".into(),
        };
        let user2 = insertion::Users {
            id: 1002,
            username: "user1002".into(),
            email: "user1002@forum.org".to_string(),
            password: "user1002rocks".into(),
        };

        let users: Result<Vec<retrieve::Users>, DbError> = em.insert(&[user1, user2]);
        println!("users: {:#?}", users);
        assert!(users.is_ok());
        panic!();
    }

    #[test]
    fn execute_sql() {
        #[derive(Debug, FromDao)]
        struct Event {
            id: i32,
            name: String,
            created: DateTime<Utc>,
        }
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let mut pool = Pool::new();
        let em = pool.em(db_url).unwrap();
        let id = 1;
        let name = "dbus-notifications".to_string();
        let created = Utc::now();
        let events: Result<Vec<Event>, DbError> = em.execute_sql_with_return(
            "SELECT $1::INT as id, $2::TEXT as name, $3::TIMESTAMP WITH TIME ZONE as created",
            &[&id, &name, &created],
        );
        println!("events: {:#?}", events);
        assert!(events.is_ok());
        for event in events.unwrap().iter() {
            assert_eq!(event.id, id);
            assert_eq!(event.name, name);
            assert_eq!(event.created.date(), created.date());
        }
    }
}
