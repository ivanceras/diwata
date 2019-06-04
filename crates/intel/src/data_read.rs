use crate::{
    common,
    data_container::{
        AppData,
        Lookup,
        QueryResult,
        RecordDetail,
        WindowData,
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

mod detail_record;

pub fn get_database_name(
    em: &EntityManager,
) -> Result<Option<DatabaseName>, DbError> {
    em.get_database_name()
}

pub fn fetch_detail(
    context: &Context,
    table_name: &TableName,
    primary_dao: &Dao,
) -> Result<RecordDetail, IntelError> {
    detail_record::get_selected_record_detail(
        context,
        table_name,
        primary_dao,
        40,
    )
}

pub fn execute_sql_query(
    context: &Context,
    sql: &str,
) -> Result<QueryResult, DbError> {
    let dialect = GenericSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql.to_string());
    println!("{:#?}", ast);
    let window = if let Ok(ast) = ast {
        if !ast.is_empty() {
            if let Some(table_name) = query_parser::extract_table_name(&ast[0])
            {
                let table_name = TableName::from(&table_name);
                context.find_window(&table_name)
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
    Ok(QueryResult::with_rows(window, rows))
}

fn fetch_main_table_data(
    context: &Context,
    table_name: &TableName,
) -> Result<Rows, IntelError> {
    let sql = format!("SELECT * FROM {} LIMIT 40", table_name.complete_name());
    let rows = context.dm.execute_sql_with_return(&sql, &[])?;
    Ok(rows)
}

pub fn retrieve_app_data(context: &Context) -> Result<AppData, IntelError> {
    let grouped_window = context.grouped_window.clone();
    let first_table_name = &grouped_window[0].window_names[0].table_name;
    let first_window = context
        .get_window(first_table_name)
        .expect("expecting a window");
    let rows = fetch_main_table_data(context, first_table_name)?;
    let first_window_data = WindowData::from_rows(rows);
    Ok(AppData {
        grouped_window,
        windows: vec![first_window.clone()],
        window_data: vec![first_window_data],
    })
}
