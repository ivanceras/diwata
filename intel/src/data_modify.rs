//! collection of functions that modify the database
//! using UPDATE and DELETE SQL

use rustorm::Value;
use rustorm::ColumnName;
use error::IntelError;
use rustorm::Table;
use rustorm::Rows;
use rustorm::DbError;
use rustorm::RecordManager;
use rustorm::Record;
use common;


/// delete the records with the following record_ids
/// return the total number of records deleted
pub fn delete_records(dm: &RecordManager,
                  main_table: &Table,
                  record_ids: &Vec<String>) -> Result<Rows, IntelError> {

    let pk_types = &main_table.get_primary_column_types();
    let primary_columns = &main_table.get_primary_column_names();
    let mut record_id_values = Vec::with_capacity(record_ids.len());
    for rid in record_ids.iter(){
        let record_id_value: Vec<(&ColumnName, Value)> = common::extract_record_id(rid, pk_types, primary_columns)?;
        record_id_values.push(record_id_value);
    }
    if primary_columns.len() == 1 {
        let rows = delete_records_from_single_primary_column(dm, main_table, &record_id_values)?;
        Ok(rows)
    }else{
        panic!("not yet handled composite primary key")
    }
}

fn delete_records_from_single_primary_column(dm: &RecordManager, main_table: &Table,
                                             record_ids: &Vec<Vec<(&ColumnName, Value)>> )-> Result<Rows, DbError>{
    let table_name = &main_table.name;
    let primary_columns = &main_table.get_primary_column_names();
    assert_eq!(primary_columns.len(), 1);
    let pk_column = primary_columns[0];
    let mut sql = format!("DELETE FROM {} ", table_name.complete_name());
    sql += &format!("WHERE {} IN (", pk_column.name);
    let mut pk_values = Vec::with_capacity(record_ids.len());
    for (i,record_id) in record_ids.iter().enumerate(){
        assert_eq!(record_id.len(), 1);
        let pk_record_id = &record_id[0];
        let pk_value = pk_record_id.1.to_owned();
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("${} ",i+1);
        pk_values.push(pk_value);
    }
    sql += ") ";
    sql += "RETURNING *";
    let rows = dm.execute_sql_with_return(&sql, &pk_values)?;
    Ok(rows)
}


/// triggered by the main tab
fn update_records_in_main_table(dm: &RecordManager, main_table: &Table,
                  rows: Rows,) -> Result<Vec<Record>,IntelError> {
    let mut records = vec![];
    for dao in rows.iter(){
        let record = Record::from(&dao);
        let updated_record = update_record_in_main_table(dm, main_table, &record)?;
        records.push(updated_record);
    }
    Ok(records)
}

fn update_record_in_main_table(dm: &RecordManager, main_table: &Table, record: &Record) 
    -> Result<Record, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("UPDATE TABLE {} ", table_name.complete_name());
    let primary_columns = &main_table.get_primary_column_names();
    for (i,pk) in primary_columns.iter().enumerate(){
        if i == 0 {
            sql += "WHERE ";
        }else{
            sql += "AND ";
        }
        sql += &format!("{} = ${} ", pk.name, i + 1);
        let pk_value = record.get_value(&pk.name);
        assert!(pk_value.is_some());
        let pk_value = pk_value.unwrap();
        params.push(pk_value);
    }
    sql += "RETURNING *";
    dm.execute_sql_with_one_return(&sql, &params)
}

/// from the main tab
fn insert_records_to_main_table(dm: &RecordManager, main_table: &Table, rows: Rows) -> Result<Vec<Record>, IntelError> {
    let mut records = vec![];
    for dao in rows.iter(){
        let record = Record::from(&dao);
        let updated_record = insert_record_to_main_table(dm, main_table, &record)?;
        records.push(updated_record);
    }
    Ok(records)
}

fn insert_record_to_main_table(dm: &RecordManager, main_table: &Table, record: &Record) -> Result<Record, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &main_table.columns;
    sql += "(";
    for (i, col) in columns.iter().enumerate(){
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("{} ", col.name.name);
    }
    sql += ") ";
    sql += "VALUES (";
    for (i, col) in columns.iter().enumerate(){
        sql += &format!("${} ", i + 1);
        let value = record.get_value(&col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        params.push(value);
    }
    sql += ") RETURNING *";
    dm.execute_sql_with_one_return(&sql, &params)
}
