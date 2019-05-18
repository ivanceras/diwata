use sqlparser::{
    sqlast::*,
    sqlparser::Parser,
};

pub(in crate) fn extract_table_name(
    sql_statement: &SQLStatement,
) -> Option<String> {
    if let SQLStatement::SQLQuery(ref sql_query) = sql_statement {
        println!("sql_query: {:#?}", sql_query);
        if let SQLSetExpr::Select(ref select) = sql_query.body {
            println!("select: {:#?}", select);
            if let Some(TableFactor::Table { ref name, .. }) = select.relation {
                println!("table: {}", name.to_string());
                Some(name.to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
