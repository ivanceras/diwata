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
use rustorm::FromDao;
use dao;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use data_container::Filter;


pub fn get_main_table<'a>(window: &Window, tables: &'a Vec<Table>) -> Option<&'a Table> {
    let main_tablename = &window.main_tab.table_name;
    let main_table = table_intel::get_table(main_tablename, tables);
    main_table
}

fn calc_offset(page: u32, page_size: u32) -> u32 {
    (page - 1) * page_size
}

pub fn get_total_records(em: &EntityManager, table_name: &TableName) -> Result<u64, DbError> {
    #[derive(FromDao)]
    struct Count{
        count: i64
    }
    let sql = format!("SELECT COUNT(*) AS count FROM {}", table_name.complete_name());
    let count: Result<Count,DbError> = em.execute_sql_with_one_return(&sql, &[]);
    count.map(|c| c.count as u64)
}

/// get data for the window
/// retrieving the Lookup table display columns
pub fn get_maintable_data(
    em: &EntityManager,
    tables: &Vec<Table>,
    window: &Window,
    filter: Option<Filter>,
    page: u32,
    page_size: u32,
) -> Result<Rows, DbError> {
    let main_table = get_main_table(window, tables);
    assert!(main_table.is_some());
    let main_table = main_table.unwrap();

    let main_tablename = &main_table.name;
    let mut sql = format!("SELECT {}.* ", main_tablename.name);

    // select the display columns of the lookup tables, left joined in this query
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    sql += &format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                }
            }
            None => (),
        }
    }

    sql += &format!("\nFROM {} \n", main_tablename.complete_name());
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                sql += &format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                );
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        sql += "\nON "
                    } else {
                        sql += "\nAND "
                    }
                    sql += &format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        main_tablename.name,
                        field_column_names[i].name
                    )
                }
            }
            None => (),
        }
    }
    let mut params = vec![];
    match filter{
        Some(filter) => {
            sql += "WHERE ";
            for (i,cond) in filter.conditions.iter().enumerate(){
                if i > 0 {
                    sql += "AND ";
                }
                let column_name = &cond.left;
                let value_str = format!("{}%", cond.right.to_string());
                let value = Value::Text(value_str);
                validate_column(&column_name, window)?;
                sql += &format!("{} ILIKE ${} ", column_name.complete_name(), i+1); 
                params.push(value);
            }

        },
        None => (),
    }
    sql += &format!("\nLIMIT {} ", page_size);
    sql += &format!("OFFSET {} ", calc_offset(page, page_size));
    println!("SQL: {}", sql);
    println!("PARAMS: {:#?}", params);
    let result: Result<Rows, DbError> = em.db().execute_sql_with_return(&sql, &params);
    println!("result: {:?}", result);
    result
}

//TODO: validate the column name here that it should exist to any of the tables
//that belong to this window, otherwise raise a SQL injection attempt error
fn validate_column(column_name: &ColumnName, window: &Window) -> Result<(), DbError>{
    if window.has_column_name(column_name){
        Ok(())
    }else{
        Err(DbError::SqlInjectionAttempt(
                    format!("Column:'{}' does not exist", column_name.complete_name())))
    }
}

/// extract record id from comma separated value
pub fn extract_record_id<'a>(
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
                            pk_type, splinter, e
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
                            pk_type, splinter, e
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
                            pk_type, splinter, e
                        )));
                    }
                }
            }
            SqlType::Numeric => {
                let v = BigDecimal::from_str(splinter);
                match v {
                    Ok(v) => Value::BigDecimal(v),
                    Err(e) => {
                        return Err(IntelError::ParamParseError(format!(
                            "Invalid for type {:?}: {}, Error: {}",
                            pk_type, splinter, e
                        )));
                    }
                }
            }

            SqlType::Varchar => {
                  Value::Text(splinter.to_string()) 
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
    let mut sql = format!("SELECT {}.* ", main_table.name.name);
    // select the display columns of the lookup tables, left joined in this query
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    sql += &format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                }
            }
            None => (),
        }
    }
    sql += &format!("\nFROM {} ", main_table.complete_name());
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                sql += &format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                );
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        sql += "\nON "
                    } else {
                        sql += "\nAND "
                    }
                    sql += &format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        main_table.name.name,
                        field_column_names[i].name
                    )
                }
            }
            None => (),
        }
    }
    let mut params: Vec<Value> = Vec::with_capacity(record_id.len());
    for (i, &(pk, ref value)) in record_id.iter().enumerate() {
        if i == 0 {
            sql += "\nWHERE ";
        } else {
            sql += "\nAND ";
        }
        sql += &format!(
            "{}.{} = ${} ",
            main_table.name.name,
            pk.complete_name(),
            i + 1
        );
        params.push(value.clone());
    }

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
            let mut indirect_records: Vec<(TableName, TableName, Rows)> =
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
                indirect_records.push((linker_table.clone(), indirect_tab.table_name.clone(), ind_records));
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
                    value, required_type
                ),
            },
            _ => panic!(
                "unsupported conversion from {:?} to {:?}",
                value, required_type
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
    let mut one_one_sql = format!("SELECT {}.* ", one_one_table.name.name);
    // select the display columns of the lookup tables, left joined in this query
    for field in &one_one_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    one_one_sql += &format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                }
            }
            None => (),
        }
    }
    one_one_sql += &format!("FROM {} ", one_one_table.complete_name());
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &one_one_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                one_one_sql += &format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                );
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        one_one_sql += "\nON "
                    } else {
                        one_one_sql += "\nAND "
                    }
                    one_one_sql += &format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        one_one_table.name.name,
                        field_column_names[i].name
                    )
                }
            }
            None => (),
        }
    }
    let referred_columns_to_main_table =
        one_one_table.get_referred_columns_to_table(&main_table.name);
    let one_one_pk = one_one_table.get_primary_column_names();
    let one_one_pk_data_types = one_one_table.get_primary_column_types();

    let mut one_one_params = Vec::with_capacity(one_one_pk.len());

    for referred_columns in referred_columns_to_main_table.iter() {
        for (i, rc) in referred_columns.iter().enumerate() {
            if i == 0 {
                one_one_sql += "\nWHERE ";
            } else {
                one_one_sql += "\nAND ";
            }
            one_one_sql += &format!(
                " {}.{} = ${} ",
                one_one_table.name.name,
                one_one_pk[i].complete_name(),
                i + 1
            );
            let required_type = one_one_pk_data_types[i];
            find_value(rc, record_id, required_type).map(|v| one_one_params.push(v.clone()));
        }
    }
    one_one_sql += &format!("\nLIMIT {} ", page_size);
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
    let mut has_many_sql = format!("SELECT {}.* ", has_many_table.name.name);

    // select the display columns of the lookup tables, left joined in this query
    for field in &has_many_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    has_many_sql += &format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                }
            }
            None => (),
        }
    }
    has_many_sql += &format!("\nFROM {} ", has_many_table.complete_name());

    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &has_many_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                has_many_sql += &format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                );
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        has_many_sql += "\nON "
                    } else {
                        has_many_sql += "\nAND "
                    }
                    has_many_sql += &format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        has_many_table.name.name,
                        field_column_names[i].name
                    )
                }
            }
            None => (),
        }
    }

    let has_many_fk = has_many_table.get_foreign_column_names_to_table(&main_table.name);
    let has_many_fk_data_types = has_many_table.get_foreign_column_types_to_table(&main_table.name);
    assert_eq!(has_many_fk.len(), has_many_fk_data_types.len());

    let mut has_many_params = Vec::with_capacity(has_many_fk.len());

    let referred_columns_to_main_table: Option<&Vec<ColumnName>> =
        has_many_table.get_referred_columns_to_table(&main_table.name);
    assert!(referred_columns_to_main_table.is_some());
    let referred_columns_to_main_table = referred_columns_to_main_table.unwrap();
    assert_eq!(referred_columns_to_main_table.len(), has_many_fk.len());

    for (i, referred_column) in referred_columns_to_main_table.iter().enumerate() {
        if i == 0 {
            has_many_sql += "\nWHERE ";
        } else {
            has_many_sql += "\nAND ";
        }
        has_many_sql += &format!(
            " {}.{} = ${} ",
            has_many_table.name.name,
            has_many_fk[i].complete_name(),
            i + 1
        );
        let required_type = has_many_fk_data_types[i];
        find_value(referred_column, main_record_id, required_type)
            .map(|v| has_many_params.push(v.clone()));
    }

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
    //let indirect_pk = indirect_table.get_primary_column_names();

    let linker_table = table_intel::get_table(linker_table_name, tables);
    assert!(linker_table.is_some());
    let linker_table = linker_table.unwrap();
    //let _linker_pk = linker_table.get_primary_column_names();
    let linker_pk_data_types = linker_table.get_primary_column_types();

    let indirect_tablename = &indirect_table.name;

    let mut indirect_sql = format!("SELECT {}.* ", indirect_tablename.name,);

    // select the display columns of the lookup tables, left joined in this query
    for field in &indirect_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    indirect_sql += &format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                }
            }
            None => (),
        }
    }
    indirect_sql += &format!("\nFROM {} ", linker_table.complete_name());
    indirect_sql += &format!("\nLEFT JOIN {} ", indirect_table.complete_name());
    println!("indirect table: {:?}", indirect_table.complete_name());
    let linker_rc_to_indirect_table =
        linker_table.get_referred_columns_to_table(indirect_tablename);
    println!(
        "indirect table referred columns to linker table: {:?} --> {:?}",
        linker_table_name.complete_name(),
        linker_rc_to_indirect_table
    );
    assert!(linker_rc_to_indirect_table.is_some());
    //let linker_rc_to_indirect_table = linker_rc_to_indirect_table.unwrap();
    let linker_fc = linker_table.get_foreign_key_to_table(indirect_tablename);
    assert!(linker_fc.is_some());
    let linker_fc = linker_fc.unwrap();
    let foreign_columns = &linker_fc.columns;
    let referring_columns = &linker_fc.referred_columns;
    println!("foreign columns: {:?}", foreign_columns);
    println!("referring column: {:?}", referring_columns);
    for (i, fc) in foreign_columns.iter().enumerate() {
        if i == 0 {
            indirect_sql += "ON "
        } else {
            indirect_sql += "AND "
        }
        indirect_sql += &format!(
            " {}.{} = {}.{} ",
            linker_table.name.name,
            fc.complete_name(),
            indirect_table.name.name,
            referring_columns[i].complete_name()
        );
    }
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &indirect_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = table_intel::get_table(&dropdown_info.source, tables);
                assert!(source_table.is_some());
                let source_table = source_table.unwrap();
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                indirect_sql += &format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                );
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        indirect_sql += "ON "
                    } else {
                        indirect_sql += "AND "
                    }
                    indirect_sql += &format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        indirect_table.name.name,
                        field_column_names[i].name
                    )
                }
            }
            None => (),
        }
    }
    println!("---> In indirect, main table: {}", main_table.name.complete_name());
    println!("---> linker table: {}", linker_table.complete_name());
    let linker_fc_to_main_table = linker_table.get_foreign_key_to_table(&main_table.name);
    println!("---> linker rc: {:#?}", linker_fc_to_main_table);
    assert!(linker_fc_to_main_table.is_some());
    let linker_fc_to_main_table = linker_fc_to_main_table.unwrap();
    let mut indirect_params =  vec![];
    let linker_fc_foreign_columns = &linker_fc_to_main_table.columns;
    let linker_fc_referring_columns = &linker_fc_to_main_table.referred_columns;
    for (i, fc) in linker_fc_foreign_columns.iter().enumerate() {
        if i == 0 {
            indirect_sql += "\nWHERE ";
        } else {
            indirect_sql += "\nAND ";
        }
        indirect_sql += &format!(
            "{}.{} = ${} ",
            linker_table.name.name,
            fc.name,
            i + 1
        );
        let required_type: &SqlType = linker_pk_data_types[i];
        let rc = &linker_fc_referring_columns[i];
        find_value(rc, record_id, required_type).map(|v| indirect_params.push(v.clone()));
    }
    indirect_sql += &format!("\nLIMIT {} ", page_size);
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
    dm: &RecordManager,
    tables: &Vec<Table>,
    window: &Window,
    page_size: u32,
) -> Result<Lookup, IntelError> {
    let mut lookup_tables = get_tab_lookup_tablenames(&window.main_tab);
    for one_one_tab in &window.one_one_tabs {
        let mut lookup = get_tab_lookup_tablenames(one_one_tab);
        lookup_tables.append(&mut lookup);
    }

    for has_many_tab in &window.has_many_tabs {
        let mut lookup = get_tab_lookup_tablenames(has_many_tab);
        lookup_tables.append(&mut lookup);
    }

    for &(ref _linker_table, ref indirect_tab) in &window.indirect_tabs {
        let mut lookup = get_tab_lookup_tablenames(indirect_tab);
        lookup_tables.append(&mut lookup);
    }
    println!("total tables: {} {:#?}", lookup_tables.len(), lookup_tables);
    //lookup_tables.dedup();
    lookup_tables.sort_by(|a,b|a.0.name.cmp(&b.0.name));
    lookup_tables.dedup_by(|a,b|a.0.name == b.0.name);
    println!("after dedup: {} {:#?}", lookup_tables.len(), lookup_tables);
    let mut lookup_data = vec![];
    for (lookup_table, display_columns) in lookup_tables {
        let rows = get_lookup_data_of_table_with_display_columns(
            dm,
            tables,
            lookup_table,
            &display_columns,
            page_size,
            1,
        )?;
        lookup_data.push((lookup_table.to_owned(), rows));
    }
    Ok(Lookup(lookup_data))
}

/// for each field of this tab that has a table lookup, get the table_name
fn get_tab_lookup_tablenames(tab: &Tab) -> Vec<(&TableName, Vec<&ColumnName>)> {
    let mut table_names = vec![];
    for field in tab.fields.iter() {
        match field.get_dropdown_info() {
            Some(dropdown_info) => {
                let display_columns = dropdown_info.display.columns.iter().collect();
                table_names.push((&dropdown_info.source, display_columns))
            }
            None => (),
        }
    }
    table_names
}

pub fn get_lookup_data_of_tab(
    dm: &RecordManager,
    tables: &Vec<Table>,
    tab: &Tab,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let table_name = &tab.table_name;
    let display_columns = tab.get_display_columns();
    get_lookup_data_of_table_with_display_columns(
        dm,
        tables,
        table_name,
        &display_columns,
        page_size,
        page,
    )
}

/// get the data of this table, no joins
/// since it is only used as lookup from some other table
/// record_id is the value that is selected in the lookup
/// ensure that the value is included in the first page
/// this table must have it's own window too
pub fn get_lookup_data_of_table_with_display_columns(
    dm: &RecordManager,
    tables: &Vec<Table>,
    table_name: &TableName,
    display_columns: &Vec<&ColumnName>,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let table = table_intel::get_table(table_name, tables);
    assert!(table.is_some());
    let table = table.unwrap();

    let primary_columns = table.get_primary_column_names();
    assert!(primary_columns.len() > 0);
    let mut sql = format!("SELECT ");
    for (i, pk) in primary_columns.iter().enumerate() {
        if i > 0 {
            sql += &","
        }
        sql += &format!("{} ", pk.name);
    }

    for column in display_columns.iter() {
        sql += &format!(", {} ", column.name);
    }
    sql += &format!("FROM {} ", table_name.complete_name());

    for (i, column) in display_columns.iter().enumerate() {
        if i == 0 {
            sql += "ORDER BY "
        } else {
            sql += ", ";
        }
        sql += &format!("{} ASC ", column.name);
    }

    sql += &format!(
        "LIMIT {} OFFSET {} ",
        page_size,
        calc_offset(page, page_size)
    );

    println!("sql: {}", sql);
    let rows = dm.execute_sql_with_return(&sql, &[])?;
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
