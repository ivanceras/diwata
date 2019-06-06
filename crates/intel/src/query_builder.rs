use crate::{
    common,
    Context,
};
use rustorm::{
    types::SqlType,
    Dao,
    DbError,
    Rows,
    Table,
    TableName,
    Value,
};
use std::collections::BTreeMap;

pub struct Query<'c> {
    context: &'c Context,
    sql: String,
    from_table: Vec<TableName>,
    joined_tables: Vec<TableName>,
    pub params: Vec<Value>,
    column_datatypes: BTreeMap<String, SqlType>,
}

impl<'c> Query<'c> {
    pub fn new(context: &'c Context) -> Self {
        Query {
            context,
            sql: String::new(),
            from_table: vec![],
            joined_tables: vec![],
            params: vec![],
            column_datatypes: BTreeMap::new(),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.sql += s;
    }

    pub fn add_param(&mut self, p: &Value) {
        let params_len = self.params.len();
        self.append(&format!("${} ", params_len + 1));
        self.params.push(p.clone());
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
        self.add_table_datatypes(table);
    }

    /// add the data types of table columns that are not part of the main tables
    /// ie. the data type of the look up tables
    pub fn add_table_datatypes(&mut self, table: &Table) {
        for column in table.columns.iter() {
            self.column_datatypes
                .insert(column.name.name.clone(), column.get_sql_type());
        }
    }

    pub fn from(&mut self, table_name: &TableName) {
        self.from_table.push(table_name.clone());
        self.append(&format!("\nFROM {} \n", table_name.safe_complete_name()));
    }

    pub fn left_join(&mut self, join_to: &TableName, join_table: &TableName) {
        assert!(!self.from_table.is_empty());
        assert!(
            self.from_table.contains(join_to)
                || self.joined_tables.contains(join_to)
        );
        let join_to = self
            .context
            .get_table(join_to)
            .expect("should have a table");
        let join_table = self
            .context
            .get_table(join_table)
            .expect("Shoul have a table");
        let local_foreign_pair =
            join_table.get_local_foreign_columns_pair_to_table(&join_to.name);
        self.append(&format!("LEFT JOIN {} ", join_table.complete_name()));

        self.joined_tables.push(join_table.name.clone());

        for (local, foreign) in local_foreign_pair {
            self.append(&format!(
                "ON {}.{} = {}.{} ",
                join_table.name.name,
                local.complete_name(),
                join_to.name.name,
                foreign.complete_name()
            ));
        }

        let local_foreign_pair2 =
            join_to.get_local_foreign_columns_pair_to_table(&join_table.name);

        for (local, foreign) in local_foreign_pair2 {
            self.append(&format!(
                "ON {}.{} = {}.{} ",
                join_to.name.name,
                local.complete_name(),
                join_table.name.name,
                foreign.complete_name()
            ));
        }
    }

    pub fn add_dao_filter(&mut self, table_name: &TableName, dao: &Dao) {
        self.append("WHERE ");
        for (column, value) in dao.0.iter() {
            self.append(&format!("{}.{} = ", table_name.name, column));
            self.add_param(value);
        }
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

    pub fn set_page(&mut self, page: usize, page_size: usize) {
        self.set_limit(page_size);
        self.append(&format!(
            "OFFSET {} ",
            common::calc_offset(page, page_size)
        ));
    }

    pub fn set_limit(&mut self, page_size: usize) {
        self.append(&format!("\nLIMIT {} ", page_size));
    }

    pub fn collect_rows(&self) -> Result<Rows, DbError> {
        println!("SQL: {}", self.sql);
        println!("params: {:?}", self.params);
        let bparams: Vec<&Value> = self.params.iter().collect();
        let result: Result<Rows, DbError> =
            self.context.dm.execute_sql_with_return(&self.sql, &bparams);
        result.map(|rows| common::cast_rows(rows, &self.column_datatypes))
    }

    pub fn collect_maybe_record(
        &self,
    ) -> Result<Option<Dao>, DbError> {
        println!("SQL: {}", self.sql);
        println!("params: {:?}", self.params);
        let bparams: Vec<&Value> = self.params.iter().collect();
        let record = self.context.dm.execute_sql_with_maybe_one_return(&self.sql, &bparams);
        record
            .map(|r| r.map(|o| common::cast_record(o, &self.column_datatypes)))
    }

    pub fn collect_one_record(&self) -> Result<Dao, DbError> {
        println!("SQL: {}", self.sql);
        println!("params: {:?}", self.params);
        let bparams: Vec<&Value> = self.params.iter().collect();
        let record = self.context.dm.execute_sql_with_one_return(&self.sql, &bparams)?;
        Ok(common::cast_record(record, &self.column_datatypes))
    }
}
