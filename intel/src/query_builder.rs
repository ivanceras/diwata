use std::collections::BTreeMap;
use rustorm::Value;
use rustorm::types::SqlType;
use rustorm::Table;
use rustorm::TableName;
use rustorm::EntityManager;
use rustorm::DbError;
use rustorm::Rows;
use rustorm::Record;
use rustorm::RecordManager;
use common;

pub struct Query{
    sql: String,
    params: Vec<Value>,
    column_datatypes: BTreeMap<String, SqlType>,
}


impl Query{

    pub fn new() -> Self{
        Query{
            sql: String::new(),
            params: vec![],
            column_datatypes: BTreeMap::new(),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.sql += s;
    }

    pub fn add_param(&mut self, p: Value) {
        self.params.push(p);
    }

    pub fn select_all(&mut self, table_name: &TableName) {
        self.append(&format!("SELECT {}.*", table_name.name));
    }

    pub fn add_table_datatypes(&mut self, table: &Table) {
        for column in table.columns.iter(){
            self.column_datatypes.insert(column.name.name.clone(), column.get_sql_type());
        }
    }

    pub fn from(&mut self, table_name: &TableName) {
        self.append(&format!("\nFROM {} \n", table_name.complete_name()));
    }

    pub fn set_page(&mut self, page: u32, page_size: u32) {
        self.append(&format!("\nLIMIT {} ", page_size));
        self.append(&format!("OFFSET {} ", common::calc_offset(page, page_size)));
    }

    pub fn collect_rows(&self, em: &EntityManager) -> Result<Rows, DbError> {
        let result: Result<Rows, DbError> = em.db().execute_sql_with_return(&self.sql, &self.params);
        result.map(|rows| common::cast_types(rows, &self.column_datatypes))
    }
    pub fn collect_rows_with_dm(&self, dm: &RecordManager) -> Result<Rows, DbError> {
        let result: Result<Rows, DbError> = dm.execute_sql_with_return(&self.sql, &self.params);
        result.map(|rows| common::cast_types(rows, &self.column_datatypes))
    }

    pub fn collect_maybe_record(&self, dm: &RecordManager) -> Result<Option<Record>, DbError> {
        let record = dm.execute_sql_with_maybe_one_return(&self.sql, &self.params);
        record.map(|r| r.map(|o| common::cast_record(o, &self.column_datatypes)))
    }
}

