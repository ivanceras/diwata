//! collection of functions that modify the database
//! using UPDATE and DELETE SQL

use common;
use data_container::RecordAction;
use data_container::RecordChangeset;
use data_container::SaveContainer;
use error::IntelError;
use rustorm;
use rustorm::ColumnName;
use rustorm::DbError;
use rustorm::Record;
use rustorm::RecordManager;
use rustorm::Rows;
use rustorm::Table;
use rustorm::TableName;
use rustorm::Value;
use tab;
use tab::Tab;
use table_intel;
use window::Window;

/// delete the records with the following record_ids
/// return the total number of records deleted
pub fn delete_records(
    dm: &RecordManager,
    main_table: &Table,
    record_ids: &Vec<String>,
) -> Result<Rows, IntelError> {
    let pk_types = &main_table.get_primary_column_types();
    let primary_columns = &main_table.get_primary_column_names();
    let mut record_id_values = Vec::with_capacity(record_ids.len());
    for rid in record_ids.iter() {
        let record_id_value: Vec<(&ColumnName, Value)> =
            common::extract_record_id(rid, pk_types, primary_columns)?;
        record_id_values.push(record_id_value);
    }
    if primary_columns.len() == 1 {
        let rows = delete_records_from_single_primary_column(dm, main_table, &record_id_values)?;
        Ok(rows)
    } else {
        panic!("not yet handled composite primary key")
    }
}

fn delete_records_from_single_primary_column(
    dm: &RecordManager,
    main_table: &Table,
    record_ids: &Vec<Vec<(&ColumnName, Value)>>,
) -> Result<Rows, DbError> {
    let table_name = &main_table.name;
    let primary_columns = &main_table.get_primary_column_names();
    assert_eq!(primary_columns.len(), 1);
    let pk_column = primary_columns[0];
    let mut sql = format!("DELETE FROM {} ", table_name.complete_name());
    sql += &format!("WHERE {} IN (", pk_column.name);
    let mut pk_values = Vec::with_capacity(record_ids.len());
    for (i, record_id) in record_ids.iter().enumerate() {
        assert_eq!(record_id.len(), 1);
        let pk_record_id = &record_id[0];
        let pk_value = pk_record_id.1.to_owned();
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("${} ", i + 1);
        pk_values.push(pk_value);
    }
    sql += ") ";
    sql += "RETURNING *";
    let rows = dm.execute_sql_with_return(&sql, &pk_values)?;
    Ok(rows)
}

pub fn save_container(
    dm: &RecordManager,
    tables: &Vec<Table>,
    container: &SaveContainer,
) -> Result<Rows, IntelError> {
    println!("container: {:#?}", container);
    let &(ref table_name_for_insert, ref rows_insert) = &container.for_insert;
    let &(ref table_name_for_update, ref rows_update) = &container.for_update;
    println!("rows_update: {:?}", rows_update);
    let table_for_insert = table_intel::get_table(table_name_for_insert, tables).unwrap();
    let table_for_update = table_intel::get_table(table_name_for_update, tables).unwrap();
    let inserted_rows = if rows_insert.iter().count() > 0 {
        insert_rows_to_table(dm, table_for_insert, rows_insert)?
    } else {
        Rows::empty()
    };
    update_records_in_table(dm, table_for_update, rows_update)?;
    Ok(inserted_rows)
}

pub fn save_changeset(
    dm: &RecordManager,
    window: &Window,
    table: &Table,
    changeset: &RecordChangeset,
) -> Result<(), IntelError> {
    println!("saving changeset: {:#?}", changeset);
    let updated_record = match &changeset.action {
        RecordAction::CreateNew => insert_record_to_table(dm, table, &changeset.record)?,
        RecordAction::Edited => update_record_in_table(dm, table, &changeset.record)?,
        _ => panic!("unhandled case: {:?}", changeset.action),
    };
    println!("updated record: {:?}", updated_record);
    save_one_ones(
        table,
        &updated_record,
        &window.one_one_tabs,
        &changeset.one_ones,
    )?;
    save_has_many(
        table,
        &updated_record,
        &window.has_many_tabs,
        &changeset.has_many,
    )?;
    save_indirect(
        table,
        &updated_record,
        &window.indirect_tabs,
        &changeset.indirect,
    )?;
    Ok(())
}

fn save_one_ones(
    main_table: &Table,
    record: &Record,
    one_one_tabs: &Vec<Tab>,
    one_one_records: &Vec<(TableName, Option<Record>)>,
) -> Result<(), IntelError> {
    println!("saving one ones: {:?}", one_one_records);
    for (one_one_table, one_one_record) in one_one_records {
        if let Some(one_one_tab) = tab::find_tab(one_one_tabs, one_one_table) {
            save_one_one_table(main_table, record, one_one_tab, one_one_record);
        }
    }
    Ok(())
}

fn save_one_one_table(
    main_Table: &Table,
    record: &Record,
    one_one_tab: &Tab,
    one_one_record: &Option<Record>,
) {
    println!("save one_one_record: {:#?}", one_one_record);
}

fn save_has_many(
    main_table: &Table,
    record: &Record,
    has_many_tabs: &Vec<Tab>,
    has_many_records: &Vec<(TableName, RecordAction, Rows)>,
) -> Result<(), IntelError> {
    println!("saving has_many : {:?}", has_many_records);
    Ok(())
}

fn save_indirect(
    main_table: &Table,
    record: &Record,
    indirect_tabs: &Vec<(TableName, Tab)>,
    indirect_records: &Vec<(TableName, TableName, RecordAction, Rows)>,
) -> Result<(), IntelError> {
    println!("saving indirect: {:?}", indirect_records);
    Ok(())
}

/// triggered by the main tab
fn update_records_in_table(
    dm: &RecordManager,
    main_table: &Table,
    rows: &Rows,
) -> Result<Vec<Record>, IntelError> {
    let mut records = vec![];
    for dao in rows.iter() {
        println!("dao: {:?}", dao);
        let record = Record::from(&dao);
        println!("record: {:?}", record);
        let updated_record = update_record_in_table(dm, main_table, &record)?;
        println!("updated record: {:?}", updated_record);
        records.push(updated_record);
    }
    Ok(records)
}

fn update_record_in_table(
    dm: &RecordManager,
    main_table: &Table,
    record: &Record,
) -> Result<Record, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("UPDATE {} ", table_name.complete_name());
    let columns = main_table.get_non_primary_columns();
    sql += "SET ";
    for (i, col) in columns.iter().enumerate() {
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("{} = ${}", col.name.name, i + 1);
        let value = record.get_value(&col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
        params.push(casted_value);
    }
    sql += " ";
    let non_pk_columns_len = columns.len();
    let primary_columns = &main_table.get_primary_columns();
    for (i, pk) in primary_columns.iter().enumerate() {
        if i == 0 {
            sql += "WHERE ";
        } else {
            sql += "AND ";
        }
        sql += &format!("{} = ${} ", pk.name.name, non_pk_columns_len + i + 1);
        let pk_value = record.get_value(&pk.name.name);
        assert!(pk_value.is_some());
        let pk_value = pk_value.unwrap();
        let casted_pk_value = rustorm::common::cast_type(&pk_value, &pk.get_sql_type());
        params.push(casted_pk_value);
    }
    sql += "RETURNING *";

    println!("sql: {}", sql);
    println!("params: {:?}", params);
    dm.execute_sql_with_one_return(&sql, &params)
}

/// insert rows all at once in one query
fn insert_rows_to_table(
    dm: &RecordManager,
    table: &Table,
    rows: &Rows,
) -> Result<Rows, IntelError> {
    let table_name = &table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &table.get_non_primary_columns();
    sql += "(";
    for (i, col) in columns.iter().enumerate() {
        if are_all_nil(&col.name.name, rows) && col.is_not_null() && col.has_generated_default() {
            println!("skipping column: {}", col.name.name);
        } else {
            if i > 0 {
                sql += ", ";
            }
            sql += &format!("{} ", col.name.name);
        }
    }
    sql += ") ";
    sql += "VALUES (";
    for dao in rows.iter() {
        for (i, col) in columns.iter().enumerate() {
            let value = dao.get_value(&col.name.name);
            assert!(value.is_some());
            let value = value.unwrap();
            if value == &Value::Nil && col.is_not_null() && col.has_generated_default() {
                println!("skipping column: {}", col.name.name);
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("${} ", i + 1);
                let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
                params.push(casted_value);
            }
        }
    }
    sql += ") RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    let rows = dm.execute_sql_with_return(&sql, &*params)?;
    Ok(rows)
}

/// check if all values in these columns are nill,
/// so we can skip it when column is skippable
fn are_all_nil(column: &str, rows: &Rows) -> bool {
    for dao in rows.iter() {
        if let Some(Value::Nil) = dao.get_value(column) {
            ;
        } else {
            return false;
        }
    }
    true
}

/// insert rows 1 by 1
fn insert_records_to_table1(
    dm: &RecordManager,
    main_table: &Table,
    rows: &Rows,
) -> Result<Vec<Record>, IntelError> {
    let mut records = vec![];
    for dao in rows.iter() {
        let record = Record::from(&dao);
        let updated_record = insert_record_to_table(dm, main_table, &record)?;
        records.push(updated_record);
    }
    Ok(records)
}

fn insert_record_to_table(
    dm: &RecordManager,
    main_table: &Table,
    record: &Record,
) -> Result<Record, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &main_table.get_non_primary_columns();
    sql += "(";
    for (i, col) in columns.iter().enumerate() {
        let value = record.get_value(&col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        if value == Value::Nil && col.is_not_null() && col.has_generated_default() {
            println!("skipping column: {}", col.name.name);
        } else {
            if i > 0 {
                sql += ", ";
            }
            sql += &format!("{} ", col.name.name);
        }
    }
    sql += ") ";
    sql += "VALUES (";
    for (i, col) in columns.iter().enumerate() {
        let value = record.get_value(&col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        if value == Value::Nil && col.is_not_null() && col.has_generated_default() {
            println!("skipping column: {}", col.name.name);
        } else {
            if i > 0 {
                sql += ", ";
            }
            sql += &format!("${} ", i + 1);
            let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
            params.push(casted_value);
        }
    }
    sql += ") RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    dm.execute_sql_with_one_return(&sql, &params)
}
