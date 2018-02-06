//! collection of functions that modify the database
//! using UPDATE and DELETE SQL

use rustorm::Value;
use rustorm::ColumnName;
use error::IntelError;
use rustorm::Table;
use rustorm::Rows;
use rustorm::DbError;
use rustorm::RecordManager;
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
fn update_records_in_main_table(_dm: &RecordManager, _main_table: &Table,
                  _rows: Rows,) -> Result<Rows,IntelError> {
    panic!("not yet");
}

/// from the main tab
fn insert_records_to_main_table(_dm: &RecordManager, _main_table: &Table, _rows: Rows) -> Result<Rows, IntelError> {
    panic!("not yet");
}
