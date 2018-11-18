use rustorm::{
    Value,
    types::SqlType,
    common,
};
use sqlparser::sqlast::ASTNode;


pub type Row = Vec<Value>;


#[derive(Debug)]
pub struct DataView{
   fields: Vec<Field>, 
   data: Vec<Row>,
}

/// the name of field and the type
#[derive(Debug)]
pub struct Field{
    pub name: String,
    pub sql_type: SqlType,
}

impl Field{

    fn convert(&self, value: &Value) -> Value {
        common::cast_type(value, &self.sql_type)
    }
}

impl DataView{

    pub fn new_from_csv(fields: Vec<Field>, csv: &str) -> Self {

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let mut field_indexes:Vec<usize> = vec![];
        for header in rdr.headers() {
            for h in header {
                if let Some(pos) = fields.iter().position(|field| field.name == h){
                    field_indexes.push(pos);
                }else{
                    panic!("undefined field: {}", h);
                }
            }
        }
        let mut tmp_rows:Vec<Row> = vec![];
        for record in rdr.records() {
            if let Ok(record) = record {
                let mut tmp_row:Row = Vec::with_capacity(fields.len());
                for value in record.iter() {
                    tmp_row.push(Value::Text(value.to_string()));
                }
                tmp_rows.push(tmp_row);
            }
        }
        let mut data:Vec<Row> = vec![];
        for tmp_row in tmp_rows{
            let mut row: Row = vec![];
            for index in &field_indexes{
                let field: &Field = &fields[*index];
                let tmp_value:&Value = &tmp_row[*index];
                let value:Value = field.convert(tmp_value);
                row.push(value);
            }
            data.push(row);
        }
        DataView{
           fields,
           data,
        }
    }


    /// add more data into this view
    fn add_page(&mut self, page: Vec<Row>) {
        for row in page{
            self.data.push(row);
        }
    }

    /// derive a view based on the sql ast
    fn get_views(&self, view_sql: &ASTNode) -> Vec<DataView>{
        vec![]
    }
}

