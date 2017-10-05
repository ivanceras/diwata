use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use value::Value;
use std::error::Error;
use error::DaoError;
use error::ConvertError;
use serde::ser::{Serialize, Serializer, SerializeStruct};


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

    fn get<T>(&'a self, s: &str) -> Result<T, DaoError<T>>
    where
        T: TryFrom<&'a Value>,
        <T as TryFrom<&'a Value>>::Error: Debug,
    {
        let value: Option<&'a Value> = self.0.get(s);
        match value {
            Some(v) => TryFrom::try_from(v).map_err(|e| DaoError::ConvertError(e)),
            None => Err(DaoError::NoSuchValueError(s.into())),
        }
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
        let expected = r#"{"lemons":{"Str":"lemonade"},"life":{"Int":42}}"#;
        assert_eq!(json, expected);
    }
}
