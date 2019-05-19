use crate::{
    common,
    data_container::{
        Lookup,
        QueryResult,
        RecordDetail,
    },
    error::IntelError,
    query_builder::Query,
    query_parser,
    tab::Tab,
    table_intel,
    window::{
        self,
        Window,
    },
    Context,
};
use rustorm::{
    types::SqlType,
    ColumnName,
    Dao,
    DaoManager,
    DatabaseName,
    DbError,
    EntityManager,
    Rows,
    Table,
    TableName,
    Value,
};
use sqlparser::{
    dialect::GenericSqlDialect,
    sqlparser::Parser,
};
use std::collections::BTreeMap;

pub fn get_main_table<'a>(
    window: &Window,
    tables: &'a [Table],
) -> Option<&'a Table> {
    let main_tablename = &window.main_tab.table_name;
    table_intel::get_table(main_tablename, tables)
}

pub fn get_database_name(
    em: &EntityManager,
) -> Result<Option<DatabaseName>, DbError> {
    em.get_database_name()
}

pub fn execute_sql_query<'a>(
    context: &Context,
    sql: String,
) -> Result<QueryResult, DbError> {
    let dialect = GenericSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql.to_string());
    println!("{:#?}", ast);
    let window = if let Ok(ast) = ast {
        if !ast.is_empty() {
            if let Some(table_name) = query_parser::extract_table_name(&ast[0])
            {
                let table_name = TableName::from(&table_name);
                let table = table_intel::get_table(&table_name, &context.tables);
                window::find_window(&table_name, &context.windows).map(Clone::clone)
            } else {
                None
            }
        } else {
            println!("Warning: there are {} statements", ast.len());
            None
        }
    } else {
        None
    };
    let rows = context.dm.execute_sql_with_return(&sql, &[])?;
    let mut rows_iter = rows.iter();
    let query_result = if rows_iter.len() == 1 {
        println!("Only 1 record, handle this...");
        let dao = rows_iter.next().expect("Expecting 1 record");
        //TODO: Also fetch the related record of this row based on the tab defined in has_many and
        // indirect tabs
        QueryResult::with_record_detail(window, RecordDetail::from_dao(dao))
    } else {
        QueryResult::with_rows(window, rows)
    };
    Ok(query_result)
}

/// get the first page of this window
pub fn get_first_page(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    window: &Window,
    page_size: i32,
) -> Result<Rows, DbError> {
    let main_table = get_main_table(window, tables).expect("Expecting a table");
    let sql = format!(
        "SELECT * FROM {} LIMIT {}",
        main_table.complete_name(),
        page_size
    );
    let rows = dm.execute_sql_with_return(&sql, &[])?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window;
    use rustorm::Pool;

    #[test]
    fn first_page() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let dm = pool.dm(db_url);
        assert!(dm.is_ok());
        let dm = dm.unwrap();
        let tables = em.get_all_tables().unwrap();
        let windows = window::derive_all_windows(&tables);
        let table_name = TableName::from("bazaar.address");
        let window = window::get_window(&table_name, &windows);
        assert!(window.is_some());
        let window = window.unwrap();
        let data =
            get_maintable_data(&em, &dm, &tables, &window, None, None, 200, 1);
        println!("data: {:#?}", data);
        assert!(data.is_ok());
    }
}
