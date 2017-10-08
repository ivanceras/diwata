use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use value::Value;
use error::DaoError;
use serde::ser::{Serialize, Serializer};


#[derive(Debug, PartialEq)]
pub struct Dao<'a>(BTreeMap<&'a str, Value>);



impl<'a> Dao<'a> {
    pub fn new() -> Self {
        Dao(BTreeMap::new())
    }

    pub fn insert<V>(&mut self, s: &'a str, v: V)
    where
        V: Into<Value>,
    {
        self.0.insert(s, v.into());
    }


    pub fn get<T>(&'a self, s: &str) -> Result<T, DaoError<T>>
    where
        T: TryFrom<&'a Value>,
        T::Error: Debug,
    {
        let value: Option<&'a Value> = self.0.get(s);
        match value {
            Some(v) => TryFrom::try_from(v).map_err(|e| DaoError::ConvertError(e)),
            None => Err(DaoError::NoSuchValueError(s.into())),
        }
    }

    pub fn remove(&mut self, s: &str) -> Option<Value> {
        self.0.remove(s)
    }
}

impl<'a> Serialize for Dao<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

pub trait FromDao {
    /// convert dao to an instance of the corresponding struct of the model
    /// taking into considerating the renamed columns
    fn from_dao(dao: &Dao) -> Self;
}

pub trait ToDao {
    /// convert from an instance of the struct to a dao representation
    /// to be saved into the database
    fn to_dao(&self) -> Dao;
}




#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use serde_json;

    #[test]
    fn insert_double() {
        let mut dao = Dao::new();
        dao.insert("life", 42.0f64);
        let life: Result<f64, DaoError<f64>> = dao.get("life");
        assert_eq!(life.unwrap(), 42.0f64);
    }

    #[test]
    fn insert_float() {
        let mut dao = Dao::new();
        dao.insert("life", 42.0f32);
        let life: Result<f64, DaoError<f64>> = dao.get("life");
        assert_eq!(life.unwrap(), 42.0f64);
    }

    #[test]
    fn uuid() {
        let mut dao = Dao::new();
        let uuid = Uuid::new_v4();
        dao.insert("user_id", uuid);
    }

    #[test]
    fn serialize_json() {
        let mut dao = Dao::new();
        dao.insert("life", 42);
        dao.insert("lemons", "lemonade");
        let json = serde_json::to_string(&dao).unwrap();
        let expected = r#"{"lemons":{"Text":"lemonade"},"life":{"Int":42}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_get_opt() {
        let mut dao = Dao::new();
        dao.insert("life", 42);
        let life: Result<Option<i32>, _> = dao.get("life");
        assert!(life.is_ok());
        let life = life.unwrap();
        assert!(life.is_some());
        assert_eq!(life.unwrap(), 42);
    }

    #[test]
    fn referenced() {
        let mut dao = Dao::new();
        let v = 42;
        let s = "lemonade";
        dao.insert("life", &v);
        dao.insert("lemons", &s);
        let life: Result<Option<i32>, _> = dao.get("life");
        assert!(life.is_ok());
        let life = life.unwrap();
        assert!(life.is_some());
        assert_eq!(life.unwrap(), 42);
    }

}
