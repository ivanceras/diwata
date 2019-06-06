//! provides data service for window
use crate::error::IntelError;
use bigdecimal::BigDecimal;
use rustorm::{
    common,
    types::SqlType,
    ColumnName,
    Dao,
    Rows,
    Value,
};
use std::{
    collections::BTreeMap,
    str::FromStr,
};
use uuid::Uuid;

#[allow(unused)]
pub fn calc_offset(page: usize, page_size: usize) -> usize {
    (page - 1) * page_size
}

pub fn cast_rows(
    rows: Rows,
    column_datatypes: &BTreeMap<String, SqlType>,
) -> Rows {
    let new_columns: Vec<String> =
        rows.columns.iter().map(ToString::to_string).collect();
    let mut casted_rows = Rows::new(new_columns);
    for dao in rows.iter() {
        let mut new_row = vec![];
        for col in rows.columns.iter() {
            let sql_type = column_datatypes.get(col);
            let value = dao.get_value(col);
            assert!(value.is_some());
            let value = value.unwrap();
            if let Some(sql_type) = sql_type {
                let casted = common::cast_type(value, sql_type);
                new_row.push(casted);
            } else {
                new_row.push(value.clone());
            }
        }
        casted_rows.push(new_row);
    }
    casted_rows
}

pub fn cast_record(
    record: Dao,
    column_datatypes: &BTreeMap<String, SqlType>,
) -> Dao {
    let mut new_rec = Dao::new();
    for (k, _v) in record.0.iter() {
        let column = k.to_string();
        let sql_type = column_datatypes.get(&column);
        let value = record.get_value(&column);
        assert!(value.is_some());
        let value = value.unwrap();
        if let Some(sql_type) = sql_type {
            let casted = common::cast_type(&value, sql_type);
            new_rec.insert_value(&column, &casted);
        }
    }
    new_rec
}

/// extract record id from comma separated value
pub fn extract_record_id<'a>(
    record_id: &str,
    pk_types: &[&SqlType],
    pk_columns: &[&'a ColumnName],
) -> Result<Vec<(&'a ColumnName, Value)>, IntelError> {
    let splinters: Vec<&str> = record_id.split(',').collect();
    let mut record_id = Vec::with_capacity(splinters.len());
    assert_eq!(splinters.len(), pk_types.len());
    assert_eq!(pk_columns.len(), pk_types.len());
    for (i, splinter) in splinters.iter().enumerate() {
        let pk_type = pk_types[i];
        let pk_column = pk_columns[i];
        let value = match *pk_type {
            SqlType::Int => {
                let v = splinter.parse();
                match v {
                    Ok(v) => Value::Int(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type, splinter, e
                        )));
                    }
                }
            }
            SqlType::Smallint => {
                let v = splinter.parse();
                match v {
                    Ok(v) => Value::Smallint(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type, splinter, e
                        )));
                    }
                }
            }
            SqlType::Uuid => {
                let uuid = Uuid::parse_str(splinter);
                match uuid {
                    Ok(uuid) => Value::Uuid(uuid),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type, splinter, e
                        )));
                    }
                }
            }
            SqlType::Numeric => {
                let v = BigDecimal::from_str(splinter);
                match v {
                    Ok(v) => Value::BigDecimal(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type, splinter, e
                        )));
                    }
                }
            }

            SqlType::Varchar => Value::Text(splinter.to_string()),
            _ => panic!("primary with type {:?} is not yet covered", pk_type),
        };
        record_id.push((pk_column, value));
    }
    Ok(record_id)
}
