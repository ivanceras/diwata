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
    pub description: Option<String>,
    pub tags: Vec<String>,
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
        let mut columns = vec![];
        if let Ok(header) = rdr.headers() {
            for h in header {
                columns.push(h)
            }
        }else{
            panic!("error reading header in csv");
        }
        let mut field_record_pos = vec![];
        for field in &fields{
            if let Some(pos) = columns.iter().position(|column| field.name == *column){
                field_record_pos.push(pos);
            }else{
                panic!("data does not have this field: {}", field.name);
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
        println!("field record pos: {:?}", field_record_pos);
        let mut data:Vec<Row> = vec![];
        for tmp_row in tmp_rows{
            let mut row: Row = vec![];
            for (i,field) in fields.iter().enumerate(){
                let index = field_record_pos[i];
                println!("index: {}", index);
                println!("field: {:?}", field);
                let tmp_value:&Value = &tmp_row[index];
                println!("tmp_value: {:?}", tmp_value);
                let value:Value = field.convert(tmp_value);
                println!("value: {:?}", value);
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

#[cfg(test)]
mod test{

    use super::*;
    
    #[test]
    fn test_from_csv(){
        let csv = r#"
pl,version,speed,vm,size,compiler
rust,1,fast,false,small,rustc
haskel,1,fast,false,small,ghc
c,99,fast,false,small,clang
java,8,medium,true,large,jdk
        "#;
        let fields = vec![
            Field{
                name: "pl".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "compiler".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "speed".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "vm".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "size".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "version".into(),
                sql_type: SqlType::Int,
            },
        ];
        let dataview = DataView::new_from_csv(fields, csv);
        assert_eq!(dataview.fields.len(), 6);
        assert_eq!(dataview.fields[0].name, "pl");
        assert_eq!(dataview.data[0][0], Value::Text("rust".to_string()));
        assert_eq!(dataview.data[1][0], Value::Text("haskel".to_string()));

        assert_eq!(dataview.fields[2].name, "speed");
        assert_eq!(dataview.data[0][2], Value::Text("fast".to_string()));
        assert_eq!(dataview.data[1][2], Value::Text("fast".to_string()));
    }
}

