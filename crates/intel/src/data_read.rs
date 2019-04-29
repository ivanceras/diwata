use crate::{
    common,
    data_container::{
        Direction,
        Filter,
        Lookup,
        Order,
        RecordDetail,
        Sort,
    },
    error::IntelError,
    query_builder::Query,
    tab::Tab,
    table_intel,
    window::Window,
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

/// get data for the window
/// retrieving the Lookup table display columns
#[allow(unused)]
#[allow(clippy::too_many_arguments)]
pub fn get_maintable_data(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    window: &Window,
    filter: Option<Filter>,
    sort: Option<Sort>,
    page: u32,
    page_size: u32,
) -> Result<Rows, DbError> {
    let main_table =
        get_main_table(window, tables).expect("main table should exist!");

    let main_tablename = &main_table.name;
    let tab = &window.main_tab;
    let mut query = Query::new();
    query.select();
    query.enumerate_columns(&main_table);

    query.add_table_datatypes(&main_table);

    query.enumerate_display_columns(&window.main_tab, tables);

    query.from(&main_tablename);
    query.left_join_display_source(&window.main_tab, tables);

    if let Some(filter) = filter {
        query.append("WHERE ");
        for (i, cond) in filter.conditions.iter().enumerate() {
            if i > 0 {
                query.append("AND ");
            }
            let column_name = &cond.left;
            let value_str = format!("{}%", cond.right.to_string());
            let value = Value::Text(value_str);
            common::validate_column(&column_name, window)?;
            query.append(&format!(
                "{}.{} ILIKE ",
                main_tablename.name, column_name.name,
            ));
            query.add_param(value);
        }
    }

    if let Some(sort) = sort {
        query.set_sort(sort);
    } else {
        // arrange by display name if there is
        if let Some(ref display) = tab.display {
            let mut orders = vec![];
            for dc in display.columns.iter() {
                let order = Order {
                    column_name: ColumnName {
                        name: dc.name.to_owned(),
                        table: Some(main_tablename.name.to_owned()),
                        alias: None,
                    },
                    direction: Direction::Asc,
                };
                orders.push(order);
            }
            query.set_sort(Sort { orders });
        }
    }
    query.set_page(page, page_size);
    let mut rows = query.collect_rows(dm)?;
    let count = em.get_total_records(main_tablename)?;
    rows.count = Some(count);
    Ok(rows)
}

/// get the detail of the selected record data
#[allow(clippy::too_many_arguments)]
pub fn get_selected_record_detail(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    window: &Window,
    record_id: &str,
    page_size: u32,
) -> Result<Option<RecordDetail>, IntelError> {
    let main_table =
        get_main_table(window, tables).expect("table should exist");
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id =
        common::extract_record_id(record_id, &pk_types, &primary_columns)?;
    println!("arg record_id: {:#?}", record_id);
    let mut query = Query::new();
    query.add_table_datatypes(&main_table);
    query.select();
    query.enumerate_columns(&main_table);
    query.enumerate_display_columns(&window.main_tab, tables);
    query.from(&main_table.name);
    query.left_join_display_source(&window.main_tab, tables);
    for (i, &(pk, ref value)) in record_id.iter().enumerate() {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!(
            "{}.{} = ",
            main_table.name.name,
            pk.complete_name(),
        ));
        query.add_param(value.clone());
    }

    let record: Option<Dao> = query.collect_maybe_record(dm)?;

    match record {
        Some(record) => {
            println!("Getting one ones");
            let mut one_one_records: Vec<(TableName, Option<Dao>)> =
                Vec::with_capacity(window.one_one_tabs.iter().count());
            for one_one_tab in window.one_one_tabs.iter() {
                let one_record = get_one_one_record(
                    em,
                    dm,
                    tables,
                    main_table,
                    one_one_tab,
                    &record_id,
                    page_size,
                )?;
                one_one_records
                    .push((one_one_tab.table_name.clone(), one_record))
            }
            let mut has_many_records: Vec<(TableName, Rows)> =
                Vec::with_capacity(window.has_many_tabs.iter().count());
            for has_many_tab in window.has_many_tabs.iter() {
                println!("Getting has many");
                let many_record = get_has_many_records(
                    em,
                    dm,
                    tables,
                    main_table,
                    has_many_tab,
                    &record_id,
                    None,
                    None,
                    page_size,
                    1,
                )?;
                println!("about to push many record: {:?}", many_record);
                has_many_records
                    .push((has_many_tab.table_name.clone(), many_record));
                println!("pushed");
            }
            println!("Getting indirect");
            let mut indirect_records: Vec<(TableName, TableName, Rows)> =
                Vec::with_capacity(window.indirect_tabs.iter().count());
            for indirect_tab in window.indirect_tabs.iter() {
                let ind_records = get_indirect_records(
                    em,
                    dm,
                    tables,
                    main_table,
                    &indirect_tab.tab,
                    &indirect_tab.linker,
                    &record_id,
                    None,
                    None,
                    page_size,
                    1,
                )?;
                indirect_records.push((
                    indirect_tab.linker.clone(),
                    indirect_tab.tab.table_name.clone(),
                    ind_records,
                ));
            }
            let detail = RecordDetail {
                record,
                one_ones: one_one_records,
                has_many: has_many_records,
                indirect: indirect_records,
            };
            Ok(Some(detail))
        }
        None => Ok(None),
    }
}

#[allow(clippy::too_many_arguments)]
fn get_one_one_record(
    _em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    main_table: &Table,
    one_one_tab: &Tab,
    record_id: &[(&ColumnName, Value)],
    page_size: u32,
) -> Result<Option<Dao>, DbError> {
    let one_one_table = table_intel::get_table(&one_one_tab.table_name, tables)
        .expect("table should exist");

    let mut query = Query::new();
    query.add_table_datatypes(&one_one_table);
    query.select();
    query.enumerate_columns(&one_one_table);

    query.enumerate_display_columns(one_one_tab, tables);
    query.from(&one_one_table.name);
    query.left_join_display_source(one_one_tab, tables);
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
                " {}.{} = ",
                one_one_table.name.name,
                one_one_pk[i].complete_name(),
            ));
            let required_type = one_one_pk_data_types[i];
            if let Some(v) = common::find_value(rc, record_id, required_type) {
                query.add_param(v.clone())
            }
        }
    }
    query.append(&format!("\nLIMIT {} ", page_size));
    query.collect_maybe_record(dm)
}

/// TODO: add filter and sort
#[allow(clippy::too_many_arguments)]
pub fn get_has_many_records_service(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    main_table: &Table,
    record_id: &str,
    has_many_tab: &Tab,
    filter: Option<Filter>,
    sort: Option<Sort>,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id =
        common::extract_record_id(record_id, &pk_types, &primary_columns)?;
    let rows = get_has_many_records(
        em,
        dm,
        tables,
        main_table,
        has_many_tab,
        &record_id,
        filter,
        sort,
        page_size,
        page,
    )?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
fn get_has_many_records(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    main_table: &Table,
    has_many_tab: &Tab,
    main_record_id: &[(&ColumnName, Value)],
    filter: Option<Filter>,
    sort: Option<Sort>,
    page_size: u32,
    page: u32,
) -> Result<Rows, DbError> {
    let has_many_table =
        table_intel::get_table(&has_many_tab.table_name, tables)
            .expect("table should exist");
    let mut query = Query::new();
    query.select();
    query.enumerate_columns(&has_many_table);

    query.add_table_datatypes(&has_many_table);
    query.enumerate_display_columns(has_many_tab, tables);
    query.from(&has_many_table.name);
    query.left_join_display_source(has_many_tab, tables);

    let has_many_fk =
        has_many_table.get_foreign_column_names_to_table(&main_table.name);
    let has_many_fk_data_types =
        has_many_table.get_foreign_column_types_to_table(&main_table.name);
    assert_eq!(has_many_fk.len(), has_many_fk_data_types.len());

    let referred_columns_to_main_table: Option<&Vec<ColumnName>> =
        has_many_table.get_referred_columns_to_table(&main_table.name);
    assert!(referred_columns_to_main_table.is_some());
    let referred_columns_to_main_table =
        referred_columns_to_main_table.unwrap();
    assert_eq!(referred_columns_to_main_table.len(), has_many_fk.len());

    for (i, referred_column) in
        referred_columns_to_main_table.iter().enumerate()
    {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!(
            " {}.{} = ",
            has_many_table.name.name,
            has_many_fk[i].complete_name(),
        ));
        let required_type = has_many_fk_data_types[i];
        if let Some(v) =
            common::find_value(referred_column, main_record_id, required_type)
        {
            query.add_param(v.clone())
        }
    }

    if let Some(filter) = filter {
        query.append("AND ");
        for (i, cond) in filter.conditions.iter().enumerate() {
            if i > 0 {
                query.append("AND ");
            }
            let column_name = &cond.left;
            let value_str = format!("{}%", cond.right.to_string());
            let value = Value::Text(value_str);
            common::validate_tab_column(&column_name, has_many_tab)?;
            query.append(&format!(
                "{}.{} ILIKE ",
                has_many_tab.table_name.name, column_name.name,
            ));
            query.add_param(value);
        }
    }

    if let Some(sort) = sort {
        query.set_sort(sort);
    } else {
        // arrange by display name if there is
        if let Some(ref display) = has_many_tab.display {
            let mut orders = vec![];
            for dc in display.columns.iter() {
                let order = Order {
                    column_name: ColumnName {
                        name: dc.name.to_owned(),
                        table: Some(has_many_tab.table_name.name.to_owned()),
                        alias: None,
                    },
                    direction: Direction::Asc,
                };
                orders.push(order);
            }
            query.set_sort(Sort { orders });
        }
    }

    query.set_page(page, page_size);
    let mut rows = query.collect_rows(dm)?;
    rows.count = Some(em.get_total_records(&has_many_table.name)?);
    Ok(rows)
}

/// TODO: add filter and sort
#[allow(clippy::too_many_arguments)]
pub fn get_indirect_records_service(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    main_table: &Table,
    record_id: &str,
    indirect_tab: &Tab,
    linker_table_name: &TableName,
    filter: Option<Filter>,
    sort: Option<Sort>,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let pk_types = main_table.get_primary_column_types();
    let primary_columns = main_table.get_primary_column_names();
    let record_id =
        common::extract_record_id(record_id, &pk_types, &primary_columns)?;
    let rows = get_indirect_records(
        em,
        dm,
        tables,
        main_table,
        indirect_tab,
        linker_table_name,
        &record_id,
        filter,
        sort,
        page_size,
        page,
    )?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
fn get_indirect_records(
    _em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    main_table: &Table,
    indirect_tab: &Tab,
    linker_table_name: &TableName,
    record_id: &[(&ColumnName, Value)],
    filter: Option<Filter>,
    sort: Option<Sort>,
    page_size: u32,
    page: u32,
) -> Result<Rows, DbError> {
    let indirect_table =
        table_intel::get_table(&indirect_tab.table_name, tables)
            .expect("table should exist");
    let mut query = Query::new();
    query.add_table_datatypes(&indirect_table);

    let linker_table = table_intel::get_table(linker_table_name, tables)
        .expect("table should exist");

    let indirect_tablename = &indirect_table.name;

    query.select();
    query.enumerate_columns(&indirect_table);
    query.enumerate_display_columns(indirect_tab, tables);
    query.from(&linker_table.name);
    query.append(&format!("\nLEFT JOIN {} ", indirect_table.complete_name()));
    let linker_rc_to_indirect_table =
        linker_table.get_referred_columns_to_table(indirect_tablename);
    assert!(linker_rc_to_indirect_table.is_some());

    let linker_fc = linker_table
        .get_foreign_key_to_table(indirect_tablename)
        .expect("table should exist");
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
    query.left_join_display_source(indirect_tab, tables);
    let linker_fc_to_main_table = linker_table
        .get_foreign_key_to_table(&main_table.name)
        .expect("table should exist");
    let linker_fc_foreign_columns = &linker_fc_to_main_table.columns;
    let linker_fc_referring_columns = &linker_fc_to_main_table.referred_columns;
    for (i, fc) in linker_fc_foreign_columns.iter().enumerate() {
        if i == 0 {
            query.append("\nWHERE ");
        } else {
            query.append("\nAND ");
        }
        query.append(&format!("{}.{} = ", linker_table.name.name, fc.name,));
        let fc_column =
            linker_table.get_column(&fc).expect("column should exist");
        let required_type: &SqlType = &fc_column.get_sql_type();
        let rc = &linker_fc_referring_columns[i];
        if let Some(v) = common::find_value(rc, record_id, required_type) {
            query.add_param(v.clone())
        }
    }
    // TODO: unify this into the query builder
    if let Some(filter) = filter {
        query.append("AND ");
        for (i, cond) in filter.conditions.iter().enumerate() {
            if i > 0 {
                query.append("AND ");
            }
            let column_name = &cond.left;
            let value_str = format!("{}%", cond.right.to_string());
            let value = Value::Text(value_str);
            common::validate_tab_column(&column_name, indirect_tab)?;
            query.append(&format!(
                "{}.{} ILIKE ",
                indirect_tab.table_name.name, column_name.name,
            ));
            query.add_param(value);
        }
    }
    if let Some(sort) = sort {
        query.set_sort(sort);
    } else {
        // arrange by display name if there is
        if let Some(ref display) = indirect_tab.display {
            let mut orders = vec![];
            for dc in display.columns.iter() {
                let order = Order {
                    column_name: ColumnName {
                        name: dc.name.to_owned(),
                        table: Some(indirect_tab.table_name.name.to_owned()),
                        alias: None,
                    },
                    direction: Direction::Asc,
                };
                orders.push(order);
            }
            query.set_sort(Sort { orders });
        }
    }
    query.set_page(page, page_size);
    query.collect_rows(dm)
}

/// for all fields in the all tabs of the window
/// that has a dropdown, fetch the first page
/// of the dropdown
#[allow(clippy::too_many_arguments)]
pub fn get_all_lookup_for_window(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    window: &Window,
    page_size: u32,
) -> Result<Lookup, IntelError> {
    let mut lookup_tables = get_tab_lookup_tablenames(&window.main_tab);
    for one_one_tab in &window.one_one_tabs {
        let mut lookup = get_tab_lookup_tablenames(one_one_tab);
        lookup_tables.append(&mut lookup);
    }

    for has_many_tab in &window.has_many_tabs {
        // treat this tab to be also a lookup for linking records in
        lookup_tables.push((
            &has_many_tab.table_name,
            has_many_tab.get_display_columns(),
        ));

        let mut lookup = get_tab_lookup_tablenames(has_many_tab);
        lookup_tables.append(&mut lookup);
    }

    for indirect_tab in &window.indirect_tabs {
        // treat this tab to be also a lookup for linking records in
        lookup_tables.push((
            &indirect_tab.tab.table_name,
            indirect_tab.tab.get_display_columns(),
        ));

        let mut lookup = get_tab_lookup_tablenames(&indirect_tab.tab);
        lookup_tables.append(&mut lookup);
    }
    println!("total tables: {} {:#?}", lookup_tables.len(), lookup_tables);
    lookup_tables.sort_by(|a, b| a.0.name.cmp(&b.0.name));
    lookup_tables.dedup_by(|a, b| a.0.name == b.0.name);
    println!("after dedup: {} {:#?}", lookup_tables.len(), lookup_tables);
    let mut lookup_data = vec![];
    for (lookup_table_name, display_columns) in lookup_tables {
        let rows = get_lookup_data_of_table_with_display_columns(
            em,
            dm,
            tables,
            lookup_table_name,
            &display_columns,
            page_size,
            1,
        )?;
        let lookup_table = table_intel::get_table(lookup_table_name, tables)
            .expect("table should exist");
        let mut column_datatypes: BTreeMap<String, SqlType> = BTreeMap::new();
        for column in lookup_table.columns.iter() {
            column_datatypes
                .insert(column.name.name.clone(), column.get_sql_type());
        }
        let rows = common::cast_rows(rows, &column_datatypes);
        lookup_data.push((lookup_table_name.to_owned(), rows));
    }
    Ok(Lookup(lookup_data))
}

/// for each field of this tab that has a table lookup, get the table_name
fn get_tab_lookup_tablenames(tab: &Tab) -> Vec<(&TableName, Vec<&ColumnName>)> {
    let mut table_names = vec![];
    for field in tab.fields.iter() {
        if let Some(dropdown_info) = field.get_dropdown_info() {
            let display_columns =
                dropdown_info.display.columns.iter().collect();
            table_names.push((&dropdown_info.source, display_columns))
        }
    }
    table_names
}

#[allow(clippy::too_many_arguments)]
pub fn get_lookup_data_of_tab(
    em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    tab: &Tab,
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let table_name = &tab.table_name;
    let display_columns = tab.get_display_columns();
    get_lookup_data_of_table_with_display_columns(
        em,
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
#[allow(clippy::too_many_arguments)]
pub fn get_lookup_data_of_table_with_display_columns(
    _em: &EntityManager,
    dm: &DaoManager,
    tables: &[Table],
    table_name: &TableName,
    display_columns: &[&ColumnName],
    page_size: u32,
    page: u32,
) -> Result<Rows, IntelError> {
    let table =
        table_intel::get_table(table_name, tables).expect("table should exist");

    let primary_columns = table.get_primary_column_names();
    //assert!(primary_columns.len() > 0);
    let mut sql = String::from("SELECT ");
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
