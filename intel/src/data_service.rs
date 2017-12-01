//! provides data service for window
use rustorm::EntityManager;
use rustorm::TableName;
use window::Window;
use window;
use rustorm::Dao;
use rustorm::Table;
use table_intel;
use rustorm::Rows;
use rustorm::DbError;
use cache;
use error::IntelError;
use rustorm::Value;
use rustorm::Column;
use rustorm::types::SqlType;
use uuid::Uuid;
use std::collections::BTreeMap;
use rustorm::Record;
use rustorm::RecordManager;
use rustorm::ColumnName;
use tab::Tab;

pub struct Filter;


fn get_main_table<'a>(window: &Window, tables: &'a Vec<Table>) -> Option<&'a Table> {
    let main_tablename = &window.main_tab.table_name;
    let main_table = table_intel::get_table(main_tablename, tables);
    main_table
}

/// get the data of the window
/// - first page data of the main table
/// - each row of the main table also loads each of 
///    its one_one record
/// - each record field that has a has_one foreign table
///    the selected value is also loaded in, additionally
///    the first page of the lookup table is also loaded
///    as well.
pub fn get_maintable_data_first_page(em: &EntityManager, 
                                 tables: &Vec<Table>,  
                                 window: &Window, 
                                 filter: Option<Filter>, 
                                 page_size: i32) -> Result<Rows, DbError> {
    let mut sql = String::from("SELECT * "); 
    let main_tablename = &window.main_tab.table_name;
    let main_table = get_main_table(window, tables);
    assert!(main_table.is_some());
    let main_table = main_table.unwrap();
    sql += &format!("FROM {} \n",main_tablename.complete_name());
    /*
    for one1 in window.one_one_tabs.iter(){
        let one1_table = table_intel::get_table(&one1.table_name, tables);
        assert!(one1_table.is_some());
        if let Some(one1_table) = one1_table{
            sql += &format!("   LEFT JOIN {} \n", one1.table_name.complete_name());
            let foreign_key = one1_table.get_foreign_key_to_table(&main_tablename);
            assert!(foreign_key.is_some());
            if let Some(fk) = foreign_key{
                assert_eq!(fk.columns.len(), fk.referred_columns.len());
                for (i, col) in fk.columns.iter().enumerate(){
                    if i == 0{
                        sql += "        ON ";
                    }else{
                        sql += "        AND ";
                    }
                    let rcol = &fk.referred_columns[i];
                    sql += &format!("{}.{} = {}.{}\n",one1.table_name.name, col.name, main_tablename.name, rcol.name) 
                }
            }
        }
    }
    for has1 in window.has_one_tables.iter(){
        let has1_table = table_intel::get_table(&has1, tables);
        assert!(has1_table.is_some());
        if let Some(has1_table) = has1_table{
            let has1_table_alias = format!("has1_{}", has1_table.name.name);
            sql += &format!("   LEFT JOIN {} AS {} \n", has1_table.complete_name(), has1_table_alias);
            let foreign_key = main_table.get_foreign_key_to_table(&has1);
            assert!(foreign_key.is_some());
            if let Some(fk) = foreign_key{
                assert_eq!(fk.columns.len(), fk.referred_columns.len());
                for (i, col) in fk.columns.iter().enumerate(){
                    if i == 0{
                        sql += "        ON ";
                    }else{
                        sql += "        AND ";
                    }
                    let rcol = &fk.referred_columns[i];
                    sql += &format!("{}.{} = {}.{}\n",main_tablename.name, col.name, 
                                    has1_table_alias, rcol.name) 
                }
            }
        }
    }
    */
    sql += &format!("LIMIT {}", page_size);
    println!("SQL: {}", sql);
    let result: Result<Rows, DbError> = em.db().execute_sql_with_return(&sql, &[]);
    println!("result: {:?}", result);
    result
}

/// extract record id from comma separated value
/// TODO: deal with edge case quoting, when there us comma in individual values
fn extract_record_id<'a>(record_id: &str, pk_types: &Vec<&SqlType>, pk_columns: &Vec<&'a ColumnName> ) -> Result<Vec<(&'a ColumnName,Value)>, IntelError> {
    let splinters:Vec<&str> = record_id.split(",").collect();
    let mut record_id = Vec::with_capacity(splinters.len());
    assert_eq!(splinters.len(), pk_types.len()); 
    assert_eq!(pk_columns.len(), pk_types.len()); 
    for (i,splinter) in splinters.iter().enumerate(){
        let pk_type = pk_types[i];
        let pk_column = pk_columns[i];
        let value = match *pk_type{
            SqlType::Int => {
                let v = splinter.parse();
                match v{
                    Ok(v) => Value::Int(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!("Invalid for type {:?}: {}",pk_type, splinter)));
                    }
                }
            }
            SqlType::Uuid => {
                let uuid = Uuid::parse_str(splinter);
                match uuid{
                    Ok(uuid) => Value::Uuid(uuid),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!("Invalid for type {:?}: {}",pk_type, splinter)));
                    }
                }
            }
            SqlType::Smallint => {
                let v = splinter.parse();
                match v{
                    Ok(v) => Value::Smallint(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!("Invalid for type {:?}: {}",pk_type, splinter)));
                    }
                }
            }
            _ => panic!("primary with type {:?} is not yet covered", pk_type)
        };
        record_id.push((pk_column, value));
    }
    Ok(record_id)
}



#[derive(Debug, Serialize)]
pub struct RecordDetail{
    pub record: Record,
    pub one_ones: Vec<(TableName, Option<Record>)>,
    pub has_many: Vec<(TableName, Rows)>,
    pub indirect: Vec<(TableName, Rows)>,
}

/// get the detail of the selected record data
pub fn get_selected_record_detail(dm: &RecordManager, tables: &Vec<Table>, 
                            window: &Window, record_id: &str) -> Result<Option<RecordDetail>, IntelError> {
    let main_table = get_main_table(window, tables);
    assert!(main_table.is_some());
    let main_table = main_table.unwrap();
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id = extract_record_id(record_id, &pk_types, &primary_columns)?;
    println!("arg record_id: {:#?}", record_id);
    let mut sql = format!("
        SELECT * FROM {} ",main_table.complete_name());
    let mut filter = "".to_string();
    let mut params: Vec<Value> = Vec::with_capacity(record_id.len());
    for (i, &(pk, ref value)) in record_id.iter().enumerate(){
        if i == 0{
            filter +="WHERE ";
        }else {
            filter += "AND ";
        }
        filter += &format!("{} = ${} ", pk.complete_name(), i+1);
        params.push(value.clone());
    }
    sql += &filter;

    println!("SQL: {}", sql);
    println!("PARAMS: {:?}", params);


    let record: Option<Record> = dm.execute_sql_with_maybe_one_return(&sql, &params)?;


    match record{
        Some(record) => {
            println!("Getting one ones");
            let mut one_one_records: Vec<(TableName, Option<Record>)> = Vec::with_capacity(window.one_one_tabs.iter().count());
            for one_one_tab in window.one_one_tabs.iter(){
                let one_record = get_one_one_record(dm, tables, main_table, one_one_tab, &record_id)?;
                one_one_records.push((one_one_tab.table_name.clone(), one_record))
            }
            let mut has_many_records: Vec<(TableName, Rows)> = Vec::with_capacity(window.has_many_tabs.iter().count());
            for has_many_tab in window.has_many_tabs.iter(){
                println!("Getting has many");
                let many_record = get_has_many_records(dm, tables, main_table, has_many_tab, &record_id)?;
                println!("about to push many record: {:?}", many_record);
                has_many_records.push((has_many_tab.table_name.clone(), many_record));
                println!("pushed");
            }
            println!("Getting indirect");
            let mut indirect_records: Vec<(TableName, Rows)> = Vec::with_capacity(window.indirect_tabs.iter().count());
            for &(ref linker_table, ref indirect_tab) in window.indirect_tabs.iter(){
                let ind_records = get_indirect_records(dm, tables, main_table, indirect_tab, linker_table, &record_id)?;
                indirect_records.push((indirect_tab.table_name.clone(), ind_records));
            }
            let detail = RecordDetail{
                record: record,
                one_ones: one_one_records, 
                has_many: has_many_records, 
                indirect: indirect_records,
            };
            Ok(Some(detail))
        }
        None => Ok(None)
    }
}

/// get the value which matches the column name and cast the value to the required data type
/// supported casting:
/// Int -> SmallInt
/// 
fn find_value<'a>(needle: &ColumnName, record_id: &'a Vec<(&ColumnName, Value)>, required_type: &SqlType) -> Option<Value> {
    record_id.iter()
        .find(|&&(ref column_name, _)| *column_name == needle )
        .map(|&(_, ref value)| cast(value, required_type))
}

fn cast(value: &Value, required_type: &SqlType) -> Value {
    if required_type.same_type(value) {
        value.to_owned()
    }
    else{
        match *value{
            Value::Int(v) => match *required_type{
                SqlType::Smallint => Value::Smallint(v as i16),
                _ => panic!("unsupported conversion from {:?} to {:?}", value, required_type)
            }
            _ => panic!("unsupported conversion from {:?} to {:?}", value, required_type)
        }
    }
}


fn get_one_one_record(dm: &RecordManager, tables: &Vec<Table>, 
                      main_table: &Table, one_one_tab: &Tab,
                      record_id: &Vec<(&ColumnName, Value)>) -> Result<Option<Record>, DbError> {
    let one_one_table = table_intel::get_table(&one_one_tab.table_name, tables);
    assert!(one_one_table.is_some());
    let one_one_table = one_one_table.unwrap();
    let one_one_tablename = &one_one_table.name;
    let mut one_one_sql = format!("SELECT * FROM {} ",one_one_table.complete_name());
    let referred_columns_to_main_table = one_one_table.get_referred_columns_to_table(&main_table.name);
    let one_one_pk = one_one_table.get_primary_column_names();
    let one_one_pk_data_types = one_one_table.get_primary_column_types();

    let mut one_one_filter = "".to_string();
    let mut one_one_params = Vec::with_capacity(one_one_pk.len());

    for referred_columns in referred_columns_to_main_table.iter(){
        for (i,rc) in referred_columns.iter().enumerate(){
            if i == 0{
                one_one_filter +="WHERE ";
            }else {
                one_one_filter += "AND ";
            }
            one_one_filter +=  &format!(" {} = ${} ", one_one_pk[i].complete_name(), i+1);
            let required_type =  one_one_pk_data_types[i];
            find_value(rc, record_id, required_type)
                .map(|v| one_one_params.push(v.clone()));
        }
    }
    one_one_sql += &one_one_filter;
    println!("referred column to main table: {:?}", referred_columns_to_main_table);
    println!("one one pk: {:?}", one_one_pk);
    println!("ONE ONE SQL: {}", one_one_sql);
    println!("ONE_ONE_PARAMS: {:?}", one_one_params);
    let one_record = dm.execute_sql_with_maybe_one_return(&one_one_sql, &one_one_params)?;
    println!("one_record: {:#?}", one_record);
    Ok(one_record)
}

fn get_has_many_records(dm: &RecordManager, tables: &Vec<Table>, 
                      main_table: &Table, has_many_tab: &Tab,
                      record_id: &Vec<(&ColumnName, Value)>) -> Result<Rows, DbError> {
    let has_many_table = table_intel::get_table(&has_many_tab.table_name, tables);
    assert!(has_many_table.is_some());
    let has_many_table = has_many_table.unwrap();
    println!("has many table: {} ", has_many_table.name.name);
    let has_many_tablename = &has_many_table.name;
    let mut has_many_sql = format!("SELECT * FROM {} ", has_many_table.complete_name());
    let has_many_fk = has_many_table.get_foreign_column_names_to_table(&main_table.name);
    let has_many_fk_data_types = has_many_table.get_foreign_column_types_to_table(&main_table.name);
    assert_eq!(has_many_fk.len(), has_many_fk_data_types.len());

    let mut has_many_filter = "".to_string();
    let mut has_many_params = Vec::with_capacity(has_many_fk.len());

    let referred_columns_to_main_table: Option<&Vec<ColumnName>> = has_many_table.get_referred_columns_to_table(&main_table.name);
    assert!(referred_columns_to_main_table.is_some());
    let referred_columns_to_main_table = referred_columns_to_main_table.unwrap();
    //TODO: Issue: has_many table may not necessarily have primary keys, but does required to have
    //foreign keys
    assert_eq!(referred_columns_to_main_table.len(), has_many_fk.len());

    for (i, referred_column) in referred_columns_to_main_table.iter().enumerate(){
        if i == 0{
            has_many_filter +="WHERE ";
        }else {
            has_many_filter += "AND ";
        }
        has_many_filter +=  &format!(" {} = ${} ", has_many_fk[i].complete_name(), i+1);
        let required_type = has_many_fk_data_types[i];
        find_value(referred_column, record_id, required_type)
            .map(|v| has_many_params.push(v.clone()));
    }

    has_many_sql += &has_many_filter;
    println!("referred column to main table: {:?}", referred_columns_to_main_table);
    println!("has_many fk: {:?}", has_many_fk);
    println!("HAS_MANY SQL: {}", has_many_sql);
    println!("HAS_MANY_PARAMS: {:?}", has_many_params);
    let rows = dm.execute_sql_with_return(&has_many_sql, &has_many_params)?;
    println!("rows: {:#?}", rows);
    Ok(rows)
}

fn get_indirect_records(dm: &RecordManager, tables: &Vec<Table>, 
                      main_table: &Table, indirect_tab: &Tab, linker_table_name: &TableName,
                      record_id: &Vec<(&ColumnName, Value)>) -> Result<Rows, DbError> {

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

    let mut indirect_sql = format!("SELECT {}.* FROM {} ", indirect_tablename.name, linker_table.complete_name());
    indirect_sql += &format!("LEFT JOIN {} ",  indirect_table.complete_name());
    println!("indirect table: {:?}", indirect_table.complete_name());
    let linker_rc_to_indirect_table = linker_table.get_referred_columns_to_table(indirect_tablename);
    println!("indirect table referred columns to linker table: {:?} --> {:?}", linker_table_name.complete_name(), linker_rc_to_indirect_table);
    assert!(linker_rc_to_indirect_table.is_some());
    let linker_rc_to_indirect_table = linker_rc_to_indirect_table.unwrap();
    for (i, rc) in linker_rc_to_indirect_table.iter().enumerate(){
        if i == 0 {
            indirect_sql +=  "ON "
        }else{
            indirect_sql += "AND "
        }
        indirect_sql += &format!(" {}.{} = {}.{} ", linker_table.name.name, rc.complete_name()
                                 , indirect_table.name.name, indirect_pk[i].complete_name());
    }
    let linker_rc_to_main_table = linker_table.get_referred_columns_to_table(&main_table.name);
    assert!(linker_rc_to_main_table.is_some());
    let linker_rc_to_main_table = linker_rc_to_main_table.unwrap();
    let mut indirect_params = Vec::with_capacity(linker_rc_to_main_table.iter().count());
    let mut filter = "".to_string();
    for (i,rc) in linker_rc_to_main_table.iter().enumerate(){
        if i == 0 {
            filter += "WHERE ";
        }else{
            filter += "AND ";
        }
        filter += &format!("{}.{} = ${} ", linker_table.name.name, linker_pk[i].complete_name(), i+1);
        let required_type: &SqlType = linker_pk_data_types[i];
        find_value(rc, record_id, required_type)
            .map(|v| indirect_params.push(v.clone()));
    }
    indirect_sql += &filter;
    println!("INDIRECT SQL: {}", indirect_sql);
    println!("INDIRECT PARAMS: {:?}", indirect_params);
    let rows = dm.execute_sql_with_return(&indirect_sql, &indirect_params)?;
    println!("rows: {:#?}", rows);
    Ok(rows)
}


/// get the next page of the window
/// the has_one record is not loaded since it is managed differently
fn get_maintable_data_next_page(em: &EntityManager, window:  &Window, filter: Option<Filter>, page: i32) {
}


/// get the data of this table, no joins
/// since it is only used as lookup from some other table
/// most likely don't request the first page since it has
/// been preloaded
fn get_lookup_data(em: &EntityManager, table_name: &TableName, 
                   filter: Option<Filter>, page: i32){
}

/// load data to a has_many tab from this window 
/// window is the window this table belongs
/// selected_record is the record on focused
/// of the main table which will be used as a filter for 
/// retrieving the data from the has_many table
/// has_many_filter is the filter for the has_many table
fn get_has_many_data(_em: &EntityManager, window: &Window, table_name: &TableName, 
                     selected_record: Dao,
                   has_many_filter: Option<Filter>,  page: i32){
}


#[cfg(test)]
mod tests{
    use super::*;
    use rustorm::Pool;

    #[test]
    fn first_page(){
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
        let window  = window.unwrap();
        let data = get_maintable_data_first_page(&em, &tables, &window, None, 200);
        println!("data: {:#?}", data);
        assert!(data.is_ok());
        let data = data.unwrap();
    }
}

