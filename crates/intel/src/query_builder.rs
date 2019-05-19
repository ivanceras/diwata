use crate::{
    common,
    tab::Tab,
    table_intel,
};
use rustorm::{
    types::SqlType,
    Dao,
    DaoManager,
    DbError,
    Rows,
    Table,
    TableName,
    Value,
};
use std::collections::BTreeMap;

pub struct Query {
    sql: String,
    pub params: Vec<Value>,
    column_datatypes: BTreeMap<String, SqlType>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            sql: String::new(),
            params: vec![],
            column_datatypes: BTreeMap::new(),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.sql += s;
    }

    pub fn add_param(&mut self, p: Value) {
        let params_len = self.params.len();
        self.append(&format!("${} ", params_len + 1));
        self.params.push(p);
    }

    pub fn select(&mut self) {
        self.append("SELECT ");
    }

    /// enumerate all column including the rename to each specific data types
    pub fn enumerate_columns(&mut self, table: &Table) {
        let columns = &table.columns;
        for (i, column) in columns.iter().enumerate() {
            if i > 0 {
                self.append(", ")
            }
            self.append(&format!(
                "{}.{}",
                &table.safe_name(),
                column.name.name
            ));
            if let Some(cast) = column.cast_as() {
                self.append(&format!("::{} ", cast.name()));
            }
        }
    }

    /// add the data types of table columns that are not part of the main tables
    /// ie. the data type of the look up tables
    pub fn add_table_datatypes(&mut self, table: &Table) {
        for column in table.columns.iter() {
            self.column_datatypes
                .insert(column.name.name.clone(), column.get_sql_type());
        }
    }

    // inlcude in the select the display columns of the lookup tables for each of the
    // fields on each tab, the source table of the field display are left joined in this query
    pub fn enumerate_display_columns(&mut self, tab: &Tab, tables: &[Table]) {
        for field in &tab.fields {
            let dropdown_info = field.get_dropdown_info();
            if let Some(ref dropdown_info) = dropdown_info {
                let source_tablename = &dropdown_info.source.name;

                let source_table =
                    table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                self.add_table_datatypes(source_table);
                if let Some(field_first_column) = &field.first_column_name() {
                    let field_column_name = &field_first_column.name;
                    let source_table_rename =
                        format!("{}_{}", field_column_name, source_tablename);
                    for display_column in &dropdown_info.display.columns {
                        let display_column_name = &display_column.name;
                        let rename = format!(
                            "{}.{}.{}",
                            field_column_name,
                            source_tablename,
                            display_column_name
                        );
                        self.append(&format!(
                            ", {}.{} as \"{}\" ",
                            source_table_rename, display_column_name, rename
                        ));
                    }
                }
            }
        }
    }

    /// left join the table that is looked up by the fields, so as to be able to retrieve the
    /// identifiable column values
    pub fn left_join_display_source(&mut self, tab: &Tab, tables: &[Table]) {
        let main_table = table_intel::get_table(&tab.table_name, tables)
            .expect("must have table");
        for field in &tab.fields {
            let dropdown_info = field.get_dropdown_info();
            if let Some(ref dropdown_info) = dropdown_info {
                let source_tablename = &dropdown_info.source.name;
                let source_table =
                    table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                if let Some(field_first_column) = field.first_column_name() {
                    let field_column_name = &field_first_column.name;
                    let field_column_names = field.column_names();
                    let source_table_rename =
                        format!("{}_{}", field_column_name, source_tablename);
                    let local_foreign_pair = main_table
                        .get_local_foreign_columns_pair_to_table(
                            &source_table.name,
                        );
                    println!("local foreign pair: {:?}", local_foreign_pair);
                    assert_eq!(source_pk.len(), field_column_names.len());
                    self.append(&format!(
                        "\nLEFT JOIN {} AS {} ",
                        &source_table.safe_complete_name(),
                        source_table_rename
                    ));
                    for (i, (local_column, source_column)) in
                        local_foreign_pair.iter().enumerate()
                    {
                        if i == 0 {
                            self.append("\nON ");
                        } else {
                            self.append("\nAND ");
                        }
                        self.append(&format!(
                            "{}.{} = {}.{} ",
                            source_table_rename,
                            source_column.name,
                            main_table.name.name,
                            local_column.name
                        ));
                    }
                }
            }
        }
    }

    pub fn from(&mut self, table_name: &TableName) {
        self.append(&format!("\nFROM {} \n", table_name.safe_complete_name()));
    }

    /*
    pub fn set_sort(&mut self, sort: Sort) {
        if !sort.orders.is_empty() {
            self.append("ORDER BY ");
            for (i, order) in sort.orders.iter().enumerate() {
                if i > 0 {
                    self.append(", ");
                }
                self.append(&format!(
                    "{} ",
                    order.column_name.safe_complete_name()
                ));
                if order.direction == Direction::Asc {
                    self.append("ASC ");
                }
                if order.direction == Direction::Desc {
                    self.append("DESC ");
                }
            }
        }
    }
    */

    pub fn set_page(&mut self, page: u32, page_size: u32) {
        self.append(&format!("\nLIMIT {} ", page_size));
        self.append(&format!(
            "OFFSET {} ",
            common::calc_offset(page, page_size)
        ));
    }

    pub fn collect_rows(&self, dm: &DaoManager) -> Result<Rows, DbError> {
        println!("SQL: {}", self.sql);
        println!("params: {:?}", self.params);
        let bparams: Vec<&Value> = self.params.iter().collect();
        let result: Result<Rows, DbError> =
            dm.execute_sql_with_return(&self.sql, &bparams);
        result.map(|rows| common::cast_rows(rows, &self.column_datatypes))
    }

    pub fn collect_maybe_record(
        &self,
        dm: &DaoManager,
    ) -> Result<Option<Dao>, DbError> {
        println!("SQL: {}", self.sql);
        println!("params: {:?}", self.params);
        let bparams: Vec<&Value> = self.params.iter().collect();
        let record = dm.execute_sql_with_maybe_one_return(&self.sql, &bparams);
        record
            .map(|r| r.map(|o| common::cast_record(o, &self.column_datatypes)))
    }
}
