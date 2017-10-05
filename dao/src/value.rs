use uuid::Uuid;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;
use error::ConvertError;


/// Generic value storage 32 byte in size
/// Some contains the same value container, but the variant is more
/// important for type hinting and view presentation hinting purposes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),

    Tinyint(i8),
    Smallint(i16),
    Int(i32),
    Bigint(i64),

    Float(f32),
    Double(f64),

    Blob(Vec<u8>),
    Text(String),
    Str(&'static str),

    Uuid(Uuid),
    Date(NaiveDate),
    Timestamp(DateTime<Utc>),
}

impl<'a> TryFrom<&'a Value> for f64 {
    type Error = ConvertError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match *value {
            Value::Double(v) => Ok(v),
            Value::Float(v) => Ok(v as f64),
            Value::Tinyint(v) => Ok(v as f64),
            Value::Smallint(v) => Ok(v as f64),
            Value::Int(v) => Ok(v as f64),
            _ => Err(ConvertError::NotSupported),
        }
    }
}

impl<'a> TryFrom<&'a Value> for f32 {
    type Error = ConvertError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match *value {
            Value::Double(v) => Ok(v as f32), //TODO: check for overflow
            Value::Float(v) => Ok(v),
            Value::Tinyint(v) => Ok(v as f32),
            Value::Smallint(v) => Ok(v as f32),
            Value::Int(v) => Ok(v as f32),
            _ => Err(ConvertError::NotSupported),
        }
    }
}


macro_rules! impl_from {
    ($ty:ty, $variant: ident) => {
        impl From<$ty> for Value {
            fn from(f: $ty) -> Self{
                Value::$variant(f)
            }
        }
    }
}

impl_from!(bool, Bool);
impl_from!(i8, Tinyint);
impl_from!(i16, Smallint);
impl_from!(i32, Int);
impl_from!(i64, Bigint);
impl_from!(f32, Float);
impl_from!(f64, Double);
impl_from!(Vec<u8>, Blob);
impl_from!(String, Text);
impl_from!(&'static str, Str);
impl_from!(Uuid, Uuid);
impl_from!(NaiveDate, Date);
impl_from!(DateTime<Utc>, Timestamp);


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn data_sizes() {
        assert_eq!(32, size_of::<Value>());
        assert_eq!(24, size_of::<Vec<u8>>());
        assert_eq!(24, size_of::<String>());
        assert_eq!(12, size_of::<DateTime<Utc>>());
        assert_eq!(4, size_of::<NaiveDate>());
        assert_eq!(16, size_of::<Uuid>());
    }

}
