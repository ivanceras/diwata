//! provides data service for window
use rustorm::EntityManager;
use rustorm::TableName;
use window::Window;
use rustorm::Table;
use table_intel;
use rustorm::Rows;
use rustorm::DbError;
use error::IntelError;
use rustorm::Value;
use rustorm::types::SqlType;
use uuid::Uuid;
use rustorm::Record;
use rustorm::RecordManager;
use rustorm::ColumnName;
use tab::Tab;
pub use data_container::RecordDetail;
use data_container::Lookup;

pub struct Filter;


pub fn get_main_table<'a>(window: &Window, tables: &'a Vec<Table>) -> Option<&'a Table> {
    let main_tablename = &window.main_tab.table_name;
    let main_table = table_intel::get_table(main_tablename, tables);
    main_table
}


fn calc_offset(page: u32, page_size: u32) -> u32 {
    (page - 1) * page_size
}

/// get data for the window
pub fn get_maintable_data(
    em: &EntityManager,
    _tables: &Vec<Table>,
    window: &Window,
    _filter: Option<Filter>,
    page: u32,
    page_size: u32,
) -> Result<Rows, DbError> {
    let mut sql = String::from("SELECT * ");
    let main_tablename = &window.main_tab.table_name;
    sql += &format!("FROM {} \n", main_tablename.complete_name());
    sql += &format!("LIMIT {} ", page_size);
    sql += &format!("OFFSET {} ", calc_offset(page, page_size));
    println!("SQL: {}", sql);
    let result: Result<Rows, DbError> = em.db().execute_sql_with_return(&sql, &[]);
    println!("result: {:?}", result);
    result
}

/// extract record id from comma separated value
/// TODO: deal with edge case quoting, when there us comma in individual values
fn extract_record_id<'a>(
    record_id: &str,
    pk_types: &Vec<&SqlType>,
    pk_columns: &Vec<&'a ColumnName>,
) -> Result<Vec<(&'a ColumnName, Value)>, IntelError> {
    let splinters: Vec<&str> = record_id.split(",").collect();
    let mut record_id = Vec::with_capacity(splinters.len());
    assert_eq!(splinters.len(), pk_types.len());
    assert_eq!(pk_columns.len(), pk_types.len());
    for (i, splinter) in splinters.iter().enumerate() {
        let pk_type = pk_types[i];
        let pk_column = pk_columns[i];
        let value = match *pk_type {
            SqlType::Int => {
                let v = splinter.parse();
                match v {
                    Ok(v) => Value::Int(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type,
                            splinter,
                            e
                        )));
                    }
                }
            }
            SqlType::Uuid => {
                let uuid = Uuid::parse_str(splinter);
                match uuid {
                    Ok(uuid) => Value::Uuid(uuid),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type,
                            splinter,
                            e
                        )));
                    }
                }
            }
            SqlType::Smallint => {
                let v = splinter.parse();
                match v {
                    Ok(v) => Value::Smallint(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type,
                            splinter,
                            e
                        )));
                    }
                }
            }
            _ => panic!("primary with type {:?} is not yet covered", pk_type),
        };
        record_id.push((pk_column, value));
    }
    Ok(record_id)
}




/// get the detail of the selected record data
pub fn get_selected_record_detail(
    dm: &RecordManager,
    tables: &Vec<Table>,
    window: &Window,
    record_id: &str,
    page_size: u32,
) -> Result<Option<RecordDetail>, IntelError> {
    let main_table = get_main_table(window, tables);
    assert!(main_table.is_some());
    let main_table = main_table.unwrap();
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id = extract_record_id(record_id, &pk_types, &primary_columns)?;
    println!("arg record_id: {:#?}", record_id);
    let mut sql = format!(
        "
        SELECT * FROM {} ",
        main_table.complete_name()
    );
    let mut filter = "".to_string();
    let mut params: Vec<Value> = Vec::with_capacity(record_id.len());
    for (i, &(pk, ref value)) in record_id.iter().enumerate() {
        if i == 0 {
            filter += "WHERE ";
        } else {
            filter += "AND ";
        }
        filter += &format!("{} = ${} ", pk.complete_name(), i + 1);
        params.push(value.clone());
    }
    sql += &filter;

    println!("SQL: {}", sql);
    println!("PARAMS: {:?}", params);


    let record: Option<Record> = dm.execute_sql_with_maybe_one_return(&sql, &params)?;


    match record {
        Some(record) => {
            println!("Getting one ones");
            let mut one_one_records: Vec<(TableName, Option<Record>)> =
                Vec::with_capacity(window.one_one_tabs.iter().count());
            for one_one_tab in window.one_one_tabs.iter() {
                let one_record =
                    get_one_one_record(dm, tables, main_table, one_one_tab, &record_id, page_size)?;
                one_one_records.push((one_one_tab.table_name.clone(), one_record))
            }
            let mut has_many_records: Vec<(TableName, Rows)> =
                Vec::with_capacity(window.has_many_tabs.iter().count());
            for has_many_tab in window.has_many_tabs.iter() {
                println!("Getting has many");
                let many_record = get_has_many_records(
                    dm,
                    tables,
                    main_table,
                    has_many_tab,
                    &record_id,
                    page_size,
                    1,
                )?;
                println!("about to push many record: {:?}", many_record);
                has_many_records.push((has_many_tab.table_name.clone(), many_record));
                println!("pushed");
            }
            println!("Getting indirect");
            let mut indirect_records: Vec<(TableName, Rows)> =
                Vec::with_capacity(window.indirect_tabs.iter().count());
            for &(ref linker_table, ref indirect_tab) in window.indirect_tabs.iter() {
                let ind_records = get_indirect_records(
                    dm,
                    tables,
                    main_table,
                    indirect_tab,
                    linker_table,
                    &record_id,
                    page_size,
                    1,
                )?;
                indirect_records.push((indirect_tab.table_name.clone(), ind_records));
            }
            let detail = RecordDetail {
                record: record,
                one_ones: one_one_records,
                has_many: has_many_records,
                indirect: indirect_records,
            };
            Ok(Some(detail))
        }
        None => Ok(None),
    }
}



/// get the value which matches the column name and cast the value to the required data type
/// supported casting:
/// Int -> SmallInt
///
fn find_value<'a>(
    needle: &ColumnName,
    record_id: &'a Vec<(&ColumnName, Value)>,
    required_type: &SqlType,
) -> Option<Value> {
    record_id
        .iter()
        .find(|&&(ref column_name, _)| *column_name == needle)
        .map(|&(_, ref value)| cast(value, required_type))
}

fn cast(value: &Value, required_type: &SqlType) -> Value {
    if required_type.same_type(value) {
        value.to_owned()
    } else {
        match *value {
            Value::Int(v) => match *required_type {
                SqlType::Smallint => Value::Smallint(v as i16),
                _ => panic!(
                    "unsupported conversion from {:?} to {:?}",
                    value,
                    required_type
                ),
            },
            _ => panic!(
                "unsupported conversion from {:?} to {:?}",
                value,
                required_type
            ),
        }
    }
}


fn get_one_one_record(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    one_one_tab: &Tab,
    record_id: &Vec<(&ColumnName, Value)>,
    page_size: u32,
) -> Result<Option<Record>, DbError> {
    let one_one_table = table_intel::get_table(&one_one_tab.table_name, tables);
    assert!(one_one_table.is_some());
    let one_one_table = one_one_table.unwrap();
    let mut one_one_sql = format!("SELECT * FROM {} ", one_one_table.complete_name());
    let referred_columns_to_main_table =
        one_one_table.get_referred_columns_to_table(&main_table.name);
    let one_one_pk = one_one_table.get_primary_column_names();
    let one_one_pk_data_types = one_one_table.get_primary_column_types();

    let mut one_one_filter = "".to_string();
    let mut one_one_params = Vec::with_capacity(one_one_pk.len());

    for referred_columns in referred_columns_to_main_table.iter() {
        for (i, rc) in referred_columns.iter().enumerate() {
            if i == 0 {
                one_one_filter += "WHERE ";
            } else {
                one_one_filter += "AND ";
            }
            one_one_filter += &format!(" {} = ${} ", one_one_pk[i].complete_name(), i + 1);
            let required_type = one_one_pk_data_types[i];
            find_value(rc, record_id, required_type).map(|v| one_one_params.push(v.clone()));
        }
    }
    one_one_sql += &one_one_filter;
    one_one_sql += &format!("LIMIT {} ", page_size);
    println!(
        "referred column to main table: {:?}",
        referred_columns_to_main_table
    );
    println!("one one pk: {:?}", one_one_pk);
    println!("ONE ONE SQL: {}", one_one_sql);
    println!("ONE_ONE_PARAMS: {:?}", one_one_params);
    let one_record = dm.execute_sql_with_maybe_one_return(&one_one_sql, &one_one_params)?;
    println!("one_record: {:#?}", one_record);
    Ok(one_record)
}

pub fn find_tab<'a>(tabs: &'a Vec<Tab>, table_name: &TableName) -> Option<&'a Tab> {
    tabs.iter().find(|tab| tab.table_name == *table_name)
}


pub fn get_has_many_records_service(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    record_id: &str,
    has_many_tab: &Tab,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id = extract_record_id(record_id, &pk_types, &primary_columns)?;
    let rows = get_has_many_records(
        dm,
        tables,
        main_table,
        has_many_tab,
        &record_id,
        page_size,
        page,
    )?;
    Ok(rows)
}



fn get_has_many_records(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    has_many_tab: &Tab,
    main_record_id: &Vec<(&ColumnName, Value)>,
    page_size: u32,
    page: u32,
) -> Result<Rows, DbError> {
    let has_many_table = table_intel::get_table(&has_many_tab.table_name, tables);
    assert!(has_many_table.is_some());
    let has_many_table = has_many_table.unwrap();
    println!("has many table: {} ", has_many_table.name.name);
    let mut has_many_sql = format!("SELECT * FROM {} ", has_many_table.complete_name());
    let has_many_fk = has_many_table.get_foreign_column_names_to_table(&main_table.name);
    let has_many_fk_data_types = has_many_table.get_foreign_column_types_to_table(&main_table.name);
    assert_eq!(has_many_fk.len(), has_many_fk_data_types.len());

    let mut has_many_filter = "".to_string();
    let mut has_many_params = Vec::with_capacity(has_many_fk.len());

    let referred_columns_to_main_table: Option<&Vec<ColumnName>> =
        has_many_table.get_referred_columns_to_table(&main_table.name);
    assert!(referred_columns_to_main_table.is_some());
    let referred_columns_to_main_table = referred_columns_to_main_table.unwrap();
    assert_eq!(referred_columns_to_main_table.len(), has_many_fk.len());

    for (i, referred_column) in referred_columns_to_main_table.iter().enumerate() {
        if i == 0 {
            has_many_filter += "WHERE ";
        } else {
            has_many_filter += "AND ";
        }
        has_many_filter += &format!(" {} = ${} ", has_many_fk[i].complete_name(), i + 1);
        let required_type = has_many_fk_data_types[i];
        find_value(referred_column, main_record_id, required_type)
            .map(|v| has_many_params.push(v.clone()));
    }

    has_many_sql += &has_many_filter;
    has_many_sql += &format!("LIMIT {} ", page_size);
    has_many_sql += &format!("OFFSET {} ", calc_offset(page, page_size));
    println!(
        "referred column to main table: {:?}",
        referred_columns_to_main_table
    );
    println!("has_many fk: {:?}", has_many_fk);
    println!("HAS_MANY SQL: {}", has_many_sql);
    println!("HAS_MANY_PARAMS: {:?}", has_many_params);
    let rows = dm.execute_sql_with_return(&has_many_sql, &has_many_params)?;
    println!("rows: {:#?}", rows);
    Ok(rows)
}

pub fn get_indirect_records_service(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    record_id: &str,
    indirect_tab: &Tab,
    linker_table_name: &TableName,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id = extract_record_id(record_id, &pk_types, &primary_columns)?;
    let rows = get_indirect_records(
        dm,
        tables,
        main_table,
        indirect_tab,
        linker_table_name,
        &record_id,
        page_size,
        page,
    )?;
    Ok(rows)
}


fn get_indirect_records(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    indirect_tab: &Tab,
    linker_table_name: &TableName,
    record_id: &Vec<(&ColumnName, Value)>,
    page_size: u32,
    page: u32,
) -> Result<Rows, DbError> {
    let indirect_table = table_intel::get_table(&indirect_tab.table_name, tables);
    assert!(indirect_table.is_some());
    let indirect_table = indirect_table.unwrap();
    let indirect_pk = indirect_table.get_primary_column_names();

    let linker_table = table_intel::get_table(linker_table_name, tables);
    assert!(linker_table.is_some());
    let linker_table = linker_table.unwrap();
    let linker_pk = linker_table.get_primary_column_names();
    let linker_pk_data_types = linker_table.get_primary_column_types();

    let indirect_tablename = &indirect_table.name;

    let mut indirect_sql = format!(
        "SELECT {}.* FROM {} ",
        indirect_tablename.name,
        linker_table.complete_name()
    );
    indirect_sql += &format!("LEFT JOIN {} ", indirect_table.complete_name());
    println!("indirect table: {:?}", indirect_table.complete_name());
    let linker_rc_to_indirect_table =
        linker_table.get_referred_columns_to_table(indirect_tablename);
    println!(
        "indirect table referred columns to linker table: {:?} --> {:?}",
        linker_table_name.complete_name(),
        linker_rc_to_indirect_table
    );
    assert!(linker_rc_to_indirect_table.is_some());
    let linker_rc_to_indirect_table = linker_rc_to_indirect_table.unwrap();
    for (i, rc) in linker_rc_to_indirect_table.iter().enumerate() {
        if i == 0 {
            indirect_sql += "ON "
        } else {
            indirect_sql += "AND "
        }
        indirect_sql += &format!(
            " {}.{} = {}.{} ",
            linker_table.name.name,
            rc.complete_name(),
            indirect_table.name.name,
            indirect_pk[i].complete_name()
        );
    }
    let linker_rc_to_main_table = linker_table.get_referred_columns_to_table(&main_table.name);
    assert!(linker_rc_to_main_table.is_some());
    let linker_rc_to_main_table = linker_rc_to_main_table.unwrap();
    let mut indirect_params = Vec::with_capacity(linker_rc_to_main_table.iter().count());
    let mut filter = "".to_string();
    for (i, rc) in linker_rc_to_main_table.iter().enumerate() {
        if i == 0 {
            filter += "WHERE ";
        } else {
            filter += "AND ";
        }
        filter += &format!(
            "{}.{} = ${} ",
            linker_table.name.name,
            linker_pk[i].complete_name(),
            i + 1
        );
        let required_type: &SqlType = linker_pk_data_types[i];
        find_value(rc, record_id, required_type).map(|v| indirect_params.push(v.clone()));
    }
    indirect_sql += &filter;
    indirect_sql += &format!("LIMIT {} ", page_size);
    indirect_sql += &format!("OFFSET {} ", calc_offset(page, page_size));
    println!("INDIRECT SQL: {}", indirect_sql);
    println!("INDIRECT PARAMS: {:?}", indirect_params);
    let rows = dm.execute_sql_with_return(&indirect_sql, &indirect_params)?;
    println!("rows: {:#?}", rows);
    Ok(rows)
}



/// for all fields in the all tabs of the window
/// that has a dropdown, fetch the first page
/// of the dropdown
pub fn get_all_lookup_for_window(
    _dm: &RecordManager,
    _tables: &Vec<Table>,
    _window: &Window) -> Lookup {
    panic!("not yet!")
}


/// get the data of this table, no joins
/// since it is only used as lookup from some other table
/// record_id is the value that is selected in the lookup
/// ensure that the value is included in the first page
/// this table must have it's own window too
pub fn get_lookup_data(
    dm: &RecordManager,
    tables: &Vec<Table>,
    tab: &Tab,
    record_id: &str,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
   
    let table_name = &tab.table_name;
    let table = table_intel::get_table(table_name, tables);
    assert!(table.is_some());
    let table = table.unwrap();

    let pk_types = table.get_primary_column_types();
    let primary_columns = table.get_primary_column_names();
    let record_id = extract_record_id(record_id, &pk_types, &primary_columns)?;

    let ident_columns = match tab.display {
        Some(ref display) => display.columns.iter().map(|ref col|*col).collect::<Vec<&ColumnName>>(),
        None => vec![]
    };
    

    let column_display: String = match tab.display{
        Some(ref display) => {
            let mut buff = "".to_string();
            let chained = primary_columns.iter()
                            .map(|col|*col)
                            .chain(display.columns.iter());
            for (i,column) in chained.enumerate(){
                if i > 0 {
                    buff += ", ";
                }
                buff += &column.name ;
            }
            buff
        }
        None => {
            "*".to_string()
        }
    };
    let mut record_sql = format!("SELECT {} FROM {} ", column_display, table_name.complete_name());
    let mut params = vec![];
    for (i, pk) in primary_columns.iter().enumerate() {
        if i == 0 {
            record_sql += "WHERE ";
        } else {
            record_sql += "AND ";
        }
        record_sql += &format!(" {} = ${} ", pk.complete_name(), i + 1);
        let required_type = pk_types[i];
        find_value(pk, &record_id, required_type)
            .map(|v| params.push(v.clone()));
    }

    let page_sql = format!("SELECT {} FROM {} ",column_display, table_name.complete_name());
    let limit_sql = &format!("LIMIT {} OFFSET {} ", page_size, calc_offset(page, page_size));
    let page_sql_with_limit = "".to_string() + &page_sql + &limit_sql;
    let mut ensured_sql = format!("WITH _$record AS ({}) ", record_sql);
    ensured_sql += &format!(", _$page AS ({})", page_sql_with_limit );
    ensured_sql += &format!(", _$ensured_page AS (
            SELECT {} FROM _$page
            UNION
            SELECT {} FROM _$record
        )", column_display, column_display);
    ensured_sql += &format!("SELECT DISTINCT {} FROM _$ensured_page ", column_display);

    let mut order_sql = "".to_string();
    for (i,column) in ident_columns.iter().enumerate(){
        if i == 0 {
            order_sql += "ORDER BY "
        }else{
            order_sql += ", ";
        }
        order_sql += &format!("{} ASC ", column.name);
    }
    
    let (sql,params) = if page == 1 {
       (ensured_sql + &order_sql , params)
    }
    else {
       (page_sql + &order_sql + &limit_sql, vec![])
    };
    println!("sql: {}", sql);
    let rows = dm.execute_sql_with_return(&sql, &params)?;
    println!("rows: {:?}", rows);
    Ok(rows)
}



#[cfg(test)]
mod tests {
    use super::*;
    use rustorm::Pool;
    use window;

    #[test]
    fn first_page() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let tables = em.get_all_tables().unwrap();
        let windows = window::derive_all_windows(&tables);
        let table_name = TableName::from("bazaar.address");
        let window = window::get_window(&table_name, &windows);
        assert!(window.is_some());
        let window = window.unwrap();
        let data = get_maintable_data(&em, &tables, &window, None, 200, 1);
        println!("data: {:#?}", data);
        assert!(data.is_ok());
    }
}
