#![deny(warnings)]
#![deny(clippy::all)]
pub use rustorm::{
    types::SqlType as Type,
    Value,
};

use rustorm::common;
use sqlparser::sqlast::ASTNode;

pub type DataRow = Vec<Value>;

/// A generic representation of rows that resembles homogeneos rows in a table
#[derive(Debug)]
pub struct DataTable {
    pub columns: Vec<DataColumn>,
    pub rows: Vec<DataRow>,
}

/// the name of field and the type
#[derive(Debug, Clone)]
pub struct DataColumn {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub data_type: Type,
    pub is_primary: bool,
}

impl DataColumn {
    fn convert(&self, value: &Value) -> Value {
        common::cast_type(value, &self.data_type)
    }
}

impl DataTable {
    pub fn from_csv(columns: Vec<DataColumn>, csv: &str) -> Self {
        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let mut column_names = vec![];
        if let Ok(header) = rdr.headers() {
            for h in header {
                column_names.push(h)
            }
        } else {
            panic!("error reading header in csv");
        }
        let mut field_record_pos = vec![];
        for field in &columns {
            if let Some(pos) =
                column_names.iter().position(|column| field.name == *column)
            {
                field_record_pos.push(pos);
            } else {
                panic!("rows does not have this field: {}", field.name);
            }
        }
        let mut tmp_rows: Vec<DataRow> = vec![];
        for record in rdr.records() {
            if let Ok(record) = record {
                let mut tmp_row: DataRow = Vec::with_capacity(columns.len());
                for value in record.iter() {
                    tmp_row.push(Value::Text(value.to_string()));
                }
                tmp_rows.push(tmp_row);
            }
        }
        println!("field record pos: {:?}", field_record_pos);
        let mut rows: Vec<DataRow> = vec![];
        for tmp_row in tmp_rows {
            let mut row: DataRow = vec![];
            for (i, field) in columns.iter().enumerate() {
                let index = field_record_pos[i];
                println!("index: {}", index);
                println!("field: {:?}", field);
                let tmp_value: &Value = &tmp_row[index];
                println!("tmp_value: {:?}", tmp_value);
                let value: Value = field.convert(tmp_value);
                println!("value: {:?}", value);
                row.push(value);
            }
            rows.push(row);
        }
        DataTable { columns, rows }
    }

    /// add more rows into this view
    pub fn add_page(&mut self, page: Vec<DataRow>) {
        for row in page {
            self.rows.push(row);
        }
    }

    /// derive a view based on the sql ast
    pub fn get_views(&self, _view_sql: &ASTNode) -> Vec<DataTable> {
        vec![]
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        Type,
        Value,
    };

    #[test]
    fn test_from_csv() {
        let csv = r#"
pl,version,speed,vm,size,compiler
rust,1,fast,false,small,rustc
haskel,1,fast,false,small,ghc
c,99,fast,false,small,clang
java,8,medium,true,large,jdk
        "#;
        let columns = vec![
            DataColumn {
                name: "pl".into(),
                data_type: Type::Text,
                description: None,
                tags: vec![],
            },
            DataColumn {
                name: "compiler".into(),
                data_type: Type::Text,
                description: None,
                tags: vec![],
            },
            DataColumn {
                name: "speed".into(),
                data_type: Type::Text,
                description: None,
                tags: vec![],
            },
            DataColumn {
                name: "vm".into(),
                data_type: Type::Text,
                description: None,
                tags: vec![],
            },
            DataColumn {
                name: "size".into(),
                data_type: Type::Text,
                description: None,
                tags: vec![],
            },
            DataColumn {
                name: "version".into(),
                data_type: Type::Int,
                description: None,
                tags: vec![],
            },
        ];
        let dataview = DataTable::from_csv(columns, csv);
        assert_eq!(dataview.columns.len(), 6);
        assert_eq!(dataview.columns[0].name, "pl");
        assert_eq!(dataview.rows[0][0], Value::Text("rust".to_string()));
        assert_eq!(dataview.rows[1][0], Value::Text("haskel".to_string()));

        assert_eq!(dataview.columns[2].name, "speed");
        assert_eq!(dataview.rows[0][2], Value::Text("fast".to_string()));
        assert_eq!(dataview.rows[1][2], Value::Text("fast".to_string()));
    }
}
