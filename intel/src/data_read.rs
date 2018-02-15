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
use rustorm::Record;
use rustorm::RecordManager;
use rustorm::ColumnName;
use tab::Tab;
pub use data_container::RecordDetail;
use data_container::Lookup;
use rustorm::FromDao;
use dao;
use data_container::Filter;
use std::collections::BTreeMap;
use common;
use query_builder::Query;


macro_rules! some {
    ($expr: expr) => {
        {
            let value = $expr;
            assert!(value.is_some());
            value.unwrap()
        }
    }
}

pub fn get_main_table<'a>(window: &Window, tables: &'a Vec<Table>) -> Option<&'a Table> {
    let main_tablename = &window.main_tab.table_name;
    let main_table = table_intel::get_table(main_tablename, tables);
    main_table
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
    let main_table = some!(get_main_table(window, tables));

    let main_tablename = &main_table.name;
    let mut query = Query::new();
    query.select_all(&main_tablename);

    query.add_table_datatypes(&main_table);

    // select the display columns of the lookup tables, left joined in this query
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                query.add_table_datatypes(source_table);

                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    let rename = format!("{}.{}.{}",
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                    query.append(&format!(
                        ", {}.{} as \"{}\" ",
                        source_table_rename,
                        display_column_name,
                        rename
                    ));
                }
            }
            None => (),
        }
    }

    query.from(&main_tablename);
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                println!("source_pk: {:?}", source_pk);
                println!("field_column_names: {:?}", field_column_names);
                assert_eq!(source_pk.len(), field_column_names.len());
                query.append(&format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                ));
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        query.append("\nON ");
                    } else {
                        query.append("\nAND ");
                    }
                    query.append(&format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        main_tablename.name,
                        field_column_names[i].name
                    ));
                }
            }
            None => (),
        }
    }
    match filter{
        Some(filter) => {
            query.append("WHERE ");
            for (i,cond) in filter.conditions.iter().enumerate(){
                if i > 0 {
                    query.append("AND ");
                }
                let column_name = &cond.left;
                let value_str = format!("{}%", cond.right.to_string());
                let value = Value::Text(value_str);
                common::validate_column(&column_name, window)?;
                query.append(&format!("{} ILIKE ${} ", column_name.complete_name(), i+1));
                query.add_param(value);
            }

        },
        None => (),
    }
    query.set_page(page, page_size);
    query.collect_rows(em)
}

/// get the detail of the selected record data
pub fn get_selected_record_detail(
    dm: &RecordManager,
    tables: &Vec<Table>,
    window: &Window,
    record_id: &str,
    page_size: u32,
) -> Result<Option<RecordDetail>, IntelError> {
    let main_table = some!(get_main_table(window, tables));
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id = common::extract_record_id(record_id, &pk_types, &primary_columns)?;
    println!("arg record_id: {:#?}", record_id);
    let mut query = Query::new();
    query.add_table_datatypes(&main_table);
    query.select_all(&main_table.name);
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
                    query.append(&format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    ));
                }
            }
            None => (),
        }
    }
    query.from(&main_table.name);
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &window.main_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;
                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                query.append(&format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                ));
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        query.append("\nON ");
                    } else {
                        query.append("\nAND ");
                    }
                    query.append(&format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        main_table.name.name,
                        field_column_names[i].name
                    ));
                }
            }
            None => (),
        }
    }
    for (i, &(pk, ref value)) in record_id.iter().enumerate() {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!(
            "{}.{} = ${} ",
            main_table.name.name,
            pk.complete_name(),
            i + 1
        ));
        query.add_param(value.clone());
    }


    let record: Option<Record> = query.collect_maybe_record(dm)?; 

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


fn get_one_one_record(
    dm: &RecordManager,
    tables: &Vec<Table>,
    main_table: &Table,
    one_one_tab: &Tab,
    record_id: &Vec<(&ColumnName, Value)>,
    page_size: u32,
) -> Result<Option<Record>, DbError> {
    let one_one_table = some!(table_intel::get_table(&one_one_tab.table_name, tables));

    let mut query = Query::new();
    query.add_table_datatypes(&one_one_table);
    query.select_all(&one_one_table.name);

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
                    query.append(&format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    ));
                }
            }
            None => (),
        }
    }
    query.from(&one_one_table.name);
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &one_one_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                query.add_table_datatypes(&source_table);

                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                query.append(&format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                ));
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        query.append("\nON ");
                    } else {
                        query.append("\nAND ");
                    }
                    query.append(&format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        one_one_table.name.name,
                        field_column_names[i].name
                    ));
                }
            }
            None => (),
        }
    }
    let referred_columns_to_main_table =
        one_one_table.get_referred_columns_to_table(&main_table.name);
    let one_one_pk = one_one_table.get_primary_column_names();
    let one_one_pk_data_types = one_one_table.get_primary_column_types();


    for referred_columns in referred_columns_to_main_table.iter() {
        for (i, rc) in referred_columns.iter().enumerate() {
            if i == 0 {
                query.append("\nWHERE ");
            } else {
                query.append("\nAND ");
            }
            query.append(&format!(
                " {}.{} = ${} ",
                one_one_table.name.name,
                one_one_pk[i].complete_name(),
                i + 1
            ));
            let required_type = one_one_pk_data_types[i];
            common::find_value(rc, record_id, required_type).map(|v| query.add_param(v.clone()));
        }
    }
    query.append(&format!("\nLIMIT {} ", page_size));
    query.collect_maybe_record(dm)
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
    let record_id = common::extract_record_id(record_id, &pk_types, &primary_columns)?;
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
    let has_many_table = some!(table_intel::get_table(&has_many_tab.table_name, tables));
    let mut query = Query::new();
    query.select_all(&has_many_table.name);

    query.add_table_datatypes(&has_many_table);

    // select the display columns of the lookup tables, left joined in this query
    for field in &has_many_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                query.add_table_datatypes(&source_table);

                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    query.append(&format!(
                        ", {}.{} as \"{}.{}.{}\" ",
                        source_table_rename,
                        display_column_name,
                        field_column_name,
                        source_tablename,
                        display_column_name
                    ));
                }
            }
            None => (),
        }
    }
    query.from(&has_many_table.name);

    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &has_many_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                query.append(&format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                ));
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        query.append("\nON ");
                    } else {
                        query.append("\nAND ");
                    }
                    query.append(&format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        has_many_table.name.name,
                        field_column_names[i].name
                    ))
                }
            }
            None => (),
        }
    }

    let has_many_fk = has_many_table.get_foreign_column_names_to_table(&main_table.name);
    let has_many_fk_data_types = has_many_table.get_foreign_column_types_to_table(&main_table.name);
    assert_eq!(has_many_fk.len(), has_many_fk_data_types.len());


    let referred_columns_to_main_table: Option<&Vec<ColumnName>> =
        has_many_table.get_referred_columns_to_table(&main_table.name);
    assert!(referred_columns_to_main_table.is_some());
    let referred_columns_to_main_table = referred_columns_to_main_table.unwrap();
    assert_eq!(referred_columns_to_main_table.len(), has_many_fk.len());

    for (i, referred_column) in referred_columns_to_main_table.iter().enumerate() {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!(
            " {}.{} = ${} ",
            has_many_table.name.name,
            has_many_fk[i].complete_name(),
            i + 1
        ));
        let required_type = has_many_fk_data_types[i];
        common::find_value(referred_column, main_record_id, required_type)
            .map(|v| query.add_param(v.clone()));
    }

    query.set_page(page, page_size);
    query.collect_rows_with_dm(dm)
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
    let record_id = common::extract_record_id(record_id, &pk_types, &primary_columns)?;
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
    let indirect_table = some!(table_intel::get_table(&indirect_tab.table_name, tables));
    let mut query = Query::new();
    query.add_table_datatypes(&indirect_table);

    let linker_table = some!(table_intel::get_table(linker_table_name, tables));
    let linker_pk_data_types = linker_table.get_primary_column_types();

    let indirect_tablename = &indirect_table.name;

    query.select_all(&indirect_tablename);

    // select the display columns of the lookup tables, left joined in this query
    for field in &indirect_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                query.add_table_datatypes(&source_table);
                let field_column_name = &field.first_column_name().name;
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                for display_column in &dropdown_info.display.columns {
                    let display_column_name = &display_column.name;
                    let rename = format!("{}.{}.{}",
                        field_column_name,
                        source_tablename,
                        display_column_name
                    );
                    query.append(&format!(
                        ", {}.{} as \"{}\" ",
                        source_table_rename,
                        display_column_name,
                        rename
                    ));
                }
            }
            None => (),
        }
    }
    query.from(&linker_table.name);
    query.append(&format!("\nLEFT JOIN {} ", indirect_table.complete_name()));
    let linker_rc_to_indirect_table =
        linker_table.get_referred_columns_to_table(indirect_tablename);
    assert!(linker_rc_to_indirect_table.is_some());

    let linker_fc = some!(linker_table.get_foreign_key_to_table(indirect_tablename));
    let foreign_columns = &linker_fc.columns;
    let referring_columns = &linker_fc.referred_columns;
    for (i, fc) in foreign_columns.iter().enumerate() {
        if i == 0 {
            query.append("ON ");
        } else {
            query.append("AND ");
        }
        query.append(&format!(
            " {}.{} = {}.{} ",
            linker_table.name.name,
            fc.complete_name(),
            indirect_table.name.name,
            referring_columns[i].complete_name()
        ));
    }
    // left join the table that is looked up by the fields, so as to be able to retrieve the
    // identifiable column values
    for field in &indirect_tab.fields {
        let dropdown_info = field.get_dropdown_info();
        match dropdown_info {
            Some(ref dropdown_info) => {
                let source_tablename = &dropdown_info.source.name;

                let source_table = some!(table_intel::get_table(&dropdown_info.source, tables));
                query.add_table_datatypes(&source_table);

                let source_pk = source_table.get_primary_column_names();
                let field_column_name = &field.first_column_name().name;
                let field_column_names = field.column_names();
                let source_table_rename = format!("{}_{}", field_column_name, source_tablename);
                assert_eq!(source_pk.len(), field_column_names.len());
                query.append(&format!(
                    "\nLEFT JOIN {} AS {} ",
                    source_table.complete_name(),
                    source_table_rename
                ));
                for (i, spk) in source_pk.iter().enumerate() {
                    if i == 0 {
                        query.append("ON ");
                    } else {
                        query.append("AND ");
                    }
                    query.append(&format!(
                        "{}.{} = {}.{} ",
                        source_table_rename,
                        spk.name,
                        indirect_table.name.name,
                        field_column_names[i].name
                    ))
                }
            }
            None => (),
        }
    }
    let linker_fc_to_main_table = some!(linker_table.get_foreign_key_to_table(&main_table.name));
    let linker_fc_foreign_columns = &linker_fc_to_main_table.columns;
    let linker_fc_referring_columns = &linker_fc_to_main_table.referred_columns;
    for (i, fc) in linker_fc_foreign_columns.iter().enumerate() {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!(
            "{}.{} = ${} ",
            linker_table.name.name,
            fc.name,
            i + 1
        ));
        let required_type: &SqlType = linker_pk_data_types[i];
        let rc = &linker_fc_referring_columns[i];
        common::find_value(rc, record_id, required_type).map(|v| query.add_param(v.clone()));
    }
    query.set_page(page, page_size);
    query.collect_rows_with_dm(dm)
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
    lookup_tables.sort_by(|a,b|a.0.name.cmp(&b.0.name));
    lookup_tables.dedup_by(|a,b|a.0.name == b.0.name);
    println!("after dedup: {} {:#?}", lookup_tables.len(), lookup_tables);
    let mut lookup_data = vec![];
    for (lookup_table_name, display_columns) in lookup_tables {
        let rows = get_lookup_data_of_table_with_display_columns(
            dm,
            tables,
            lookup_table_name,
            &display_columns,
            page_size,
            1,
        )?;
        let lookup_table = some!(table_intel::get_table(lookup_table_name, tables));
        let mut column_datatypes:BTreeMap<String, SqlType> = BTreeMap::new();
        for column in lookup_table.columns.iter(){
            column_datatypes.insert(column.name.name.clone(), column.get_sql_type());
        }
        let rows = common::cast_types(rows, &column_datatypes);
        lookup_data.push((lookup_table_name.to_owned(), rows));
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
    let table = some!(table_intel::get_table(table_name, tables));

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
        common::calc_offset(page, page_size)
    );

    println!("sql: {}", sql);
    let rows = dm.execute_sql_with_return(&sql, &[])?;
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
