use crate::{
    data_container::RecordDetail,
    error::IntelError,
    query_builder::Query,
    Context,
    TableName,
    Window,
};
use rustorm::{
    Dao,
    DaoManager,
    EntityManager,
    Rows,
    Table,
};

/// get the detail of the selected record data
pub fn get_selected_record_detail(
    context: &Context,
    table_name: &TableName,
    primary_dao: &Dao,
    page_size: usize,
) -> Result<RecordDetail, IntelError> {
    let window = context
        .get_window(table_name)
        .expect("should have a window");
    let main_table =
        context.get_table(table_name).expect("should have a table");

    let mut query = Query::new(context);
    query.add_table_datatypes(&main_table);
    query.select();
    query.enumerate_columns(&main_table);
    query.from(&main_table.name);
    query.add_dao_filter(&main_table.name, primary_dao);

    let record: Dao = query.collect_one_record(&context.dm)?;

    println!("Getting one ones");
    let mut one_one_records: Vec<(TableName, Option<Dao>)> =
        Vec::with_capacity(window.one_one_tabs.iter().count());
    for one_one_tab in window.one_one_tabs.iter() {
        let one_record = get_one_one_record(
            context,
            main_table,
            &one_one_tab.table_name,
            primary_dao,
            page_size,
        )?;
        println!("one one record: {:#?}", one_record);
        one_one_records.push((one_one_tab.table_name.clone(), one_record))
    }
    let mut has_many_records: Vec<(TableName, Rows)> =
        Vec::with_capacity(window.has_many_tabs.iter().count());
    for has_many_tab in window.has_many_tabs.iter() {
        println!("Getting has many");
        let many_record = get_has_many_records(
            context,
            main_table,
            &has_many_tab.table_name,
            primary_dao,
            page_size,
        )?;
        println!("about to push many record: {:?}", many_record);
        has_many_records.push((has_many_tab.table_name.clone(), many_record));
        println!("pushed");
    }
    println!("Getting indirect");
    let mut indirect_records: Vec<(TableName, TableName, Rows)> =
        Vec::with_capacity(window.indirect_tabs.iter().count());
    for indirect_tab in window.indirect_tabs.iter() {
        let ind_records = get_indirect_records(
            context,
            main_table,
            &indirect_tab.tab.table_name,
            &indirect_tab.linker,
            primary_dao,
            page_size,
        )?;
        indirect_records.push((
            indirect_tab.linker.clone(),
            indirect_tab.tab.table_name.clone(),
            ind_records,
        ));
    }

    Ok(RecordDetail {
        record,
        one_ones: one_one_records,
        has_many: has_many_records,
        indirect: indirect_records,
    })
}

fn get_one_one_record(
    context: &Context,
    main_table: &Table,
    one_one_table_name: &TableName,
    primary_dao: &Dao,
    page_size: usize,
) -> Result<Option<Dao>, IntelError> {
    let one_one_table = context
        .get_table(one_one_table_name)
        .expect("should matched a table");
    let mut query = Query::new(context);
    query.add_table_datatypes(&one_one_table);
    query.select();
    query.enumerate_columns(&one_one_table);
    query.from(&main_table.name);
    query.left_join(&main_table.name, &one_one_table.name);
    query.add_dao_filter(&main_table.name, &primary_dao);
    query.set_limit(page_size);
    let one_one_record = query.collect_maybe_record(&context.dm)?;
    Ok(one_one_record)
}

fn get_has_many_records(
    context: &Context,
    main_table: &Table,
    has_many_table_name: &TableName,
    primary_dao: &Dao,
    page_size: usize,
) -> Result<Rows, IntelError> {
    let has_many_table = context
        .get_table(has_many_table_name)
        .expect("table should exist");
    let mut query = Query::new(context);
    query.select();
    query.enumerate_columns(&has_many_table);

    query.add_table_datatypes(&has_many_table);
    query.from(&main_table.name);
    query.left_join(&main_table.name, &has_many_table.name);
    query.add_dao_filter(&main_table.name, primary_dao);
    query.set_limit(page_size);
    let mut rows = query.collect_rows(&context.dm)?;
    rows.count = Some(context.em.get_total_records(&has_many_table.name)?);
    Ok(rows)
}

fn get_indirect_records(
    context: &Context,
    main_table: &Table,
    indirect_table_name: &TableName,
    linker_table: &TableName,
    primary_dao: &Dao,
    page_size: usize,
) -> Result<Rows, IntelError> {

    let indirect_table = context
        .get_table(indirect_table_name)
        .expect("table should exist");
    let mut query = Query::new(context);
    query.select();
    query.enumerate_columns(&indirect_table);

    query.add_table_datatypes(&indirect_table);
    query.from(&main_table.name);
    query.left_join(&main_table.name, &linker_table);
    query.left_join(&linker_table, &indirect_table.name);
    query.add_dao_filter(&main_table.name, primary_dao);
    query.set_limit(page_size);
    let mut rows = query.collect_rows(&context.dm)?;
    rows.count = Some(context.em.get_total_records(&indirect_table.name)?);
    Ok(rows)
}
