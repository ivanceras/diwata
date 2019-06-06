use crate::{
    data_container::{
        AppData,
        QueryResult,
        RecordDetail,
        WindowData,
    },
    error::IntelError,
    query_builder::Query,
    query_parser,
    Context,
};
use rustorm::{
    Dao,
    DatabaseName,
    DbError,
    EntityManager,
    Rows,
    TableName,
};
use sqlparser::{
    dialect::GenericSqlDialect,
    sqlparser::Parser,
};


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


pub fn get_window_main_table_data(
    context: &Context,
    table_name: &TableName,
) -> Result<QueryResult, IntelError> {
    let window = context.get_window(table_name);
    let rows = fetch_main_table_data(context, table_name)?;
    Ok(QueryResult::with_rows(window, rows))
}

fn fetch_main_table_data(
    context: &Context,
    table_name: &TableName,
) -> Result<Rows, IntelError> {
    let mut query = Query::new(context);
    query.select();
    let main_table = context.get_table(table_name).expect("there should be table");
    query.enumerate_columns(&main_table);
    query.from(table_name);
    query.set_limit(40);
    let rows = query.collect_rows()?;
    Ok(rows)
}

pub fn retrieve_app_data(context: &Context, table_name: Option<TableName>) -> Result<AppData, IntelError> {
    let grouped_window = context.grouped_window.clone();
    println!("table_name: {:#?}", table_name);
    let retrieve_table_name = if let Some(ref table_name) = &table_name{
        table_name
    }else{
        &grouped_window[0].window_names[0].table_name
    };
    let first_window = context
        .get_window(retrieve_table_name)
        .expect("expecting a window");
    let rows = fetch_main_table_data(context, retrieve_table_name)?;
    let first_window_data = WindowData::from_rows(rows);
    Ok(AppData {
        grouped_window,
        windows: vec![first_window.clone()],
        window_data: vec![first_window_data],
    })
}
