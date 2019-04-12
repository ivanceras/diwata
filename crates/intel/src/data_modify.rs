//! collection of functions that modify the database
//! using UPDATE and DELETE SQL

use crate::common;
use crate::data_container::RecordAction;
use crate::data_container::RecordChangeset;
use crate::data_container::SaveContainer;
use crate::error::IntelError;
use crate::tab;
use crate::tab::Tab;
use crate::table_intel;
use crate::window::Window;
use rustorm;
use rustorm::ColumnName;
use rustorm::Dao;
use rustorm::DaoManager;
use rustorm::DbError;
use rustorm::Rows;
use rustorm::Table;
use rustorm::TableName;
use rustorm::Value;

/// delete the records with the following record_ids
/// return the total number of records deleted
pub fn delete_records(
    dm: &DaoManager,
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
    dm: &DaoManager,
    main_table: &Table,
    record_ids: &Vec<Vec<(&ColumnName, Value)>>,
) -> Result<Rows, DbError> {
    let table_name = &main_table.name;
    let primary_columns = &main_table.get_primary_column_names();
    assert_eq!(primary_columns.len(), 1);
    let pk_column = primary_columns[0];
    let mut sql = format!("DELETE FROM {} ", table_name.complete_name());
    sql += &format!("WHERE {} IN (", pk_column.name);
    let mut pk_values: Vec<Value> = Vec::with_capacity(record_ids.len());
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
    let bpk_values: Vec<&Value> = pk_values.iter().collect();
    let rows = dm.execute_sql_with_return(&sql, &bpk_values)?;
    Ok(rows)
}

pub fn save_container(
    dm: &DaoManager,
    tables: &Vec<Table>,
    container: &SaveContainer,
) -> Result<(), IntelError> {
    let &(ref table_name_for_insert, ref rows_insert) = &container.for_insert;
    let &(ref table_name_for_update, ref rows_update) = &container.for_update;
    let table_for_insert = table_intel::get_table(table_name_for_insert, tables).unwrap();
    let table_for_update = table_intel::get_table(table_name_for_update, tables).unwrap();
    if rows_insert.iter().count() > 0 {
        insert_rows_to_table(dm, table_for_insert, rows_insert)?;
    }
    update_records_in_table(dm, table_for_update, rows_update)?;
    Ok(())
}

pub fn save_changeset(
    dm: &DaoManager,
    tables: &Vec<Table>,
    window: &Window,
    table: &Table,
    changeset: &RecordChangeset,
) -> Result<(), IntelError> {
    let updated_record = match &changeset.action {
        RecordAction::CreateNew => insert_record_to_table(dm, table, &changeset.record)?,
        RecordAction::Edited => update_record_in_table(dm, table, &changeset.record)?,
        _ => panic!("unhandled case: {:?}", changeset.action),
    };
    save_one_ones(
        dm,
        tables,
        table,
        &updated_record,
        &window.one_one_tabs,
        &changeset.one_ones,
    )?;
    save_has_many(
        dm,
        tables,
        table,
        &updated_record,
        &window.has_many_tabs,
        &changeset.has_many,
    )?;
    save_indirect(
        dm,
        tables,
        table,
        &updated_record,
        &window.indirect_tabs,
        &changeset.indirect,
    )?;
    Ok(())
}

fn save_one_ones(
    dm: &DaoManager,
    tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    _one_one_tabs: &Vec<Tab>,
    one_one_records: &Vec<(TableName, Option<Dao>)>,
) -> Result<(), IntelError> {
    for (one_one_table_name, one_one_record) in one_one_records {
        if let Some(one_one_record) = one_one_record {
            //TODO: verify that the one_one_table_name belongs to the one_one_tabs
            if let Some(one_one_table) = table_intel::get_table(one_one_table_name, tables) {
                save_one_one_table(
                    dm,
                    tables,
                    main_table,
                    main_record,
                    one_one_table,
                    one_one_record,
                )?;
            }
        }
    }
    Ok(())
}

fn save_one_one_table(
    dm: &DaoManager,
    _tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    one_one_table: &Table,
    one_one_record: &Dao,
) -> Result<Dao, DbError> {
    upsert_one_one_record_to_table(dm, main_table, main_record, one_one_table, one_one_record)
}

fn save_has_many(
    dm: &DaoManager,
    tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    has_many_tabs: &Vec<Tab>,
    has_many_records: &Vec<(TableName, RecordAction, Rows)>,
) -> Result<(), IntelError> {
    for (has_many_table_name, record_action, has_many_rows) in has_many_records {
        let _has_many_tab = tab::find_tab(has_many_tabs, has_many_table_name)
            .expect("table should belong to the tabs");
        let has_many_table =
            table_intel::get_table(has_many_table_name, tables).expect("table should exist");
        save_has_many_table(
            dm,
            tables,
            main_table,
            main_record,
            has_many_table,
            record_action,
            &has_many_rows,
        )?;
    }
    Ok(())
}

fn save_has_many_table(
    dm: &DaoManager,
    _tables: &Vec<Table>,
    _main_table: &Table,
    _main_record: &Dao,
    has_many_table: &Table,
    record_action: &RecordAction,
    has_many_rows: &Rows,
) -> Result<(), IntelError> {
    match record_action {
        RecordAction::Unlink => {
            delete_from_table(dm, has_many_table, has_many_rows)?;
        }
        RecordAction::LinkNew => {
            if has_many_rows.iter().count() > 0 {
                insert_rows_to_table(dm, has_many_table, has_many_rows)?;
            }
        }
        RecordAction::Edited => {
            update_records_in_table(dm, has_many_table, has_many_rows)?;
        }
        _ => panic!("unexpected record action: {:?} in has_many", record_action),
    }
    Ok(())
}

fn delete_from_table(dm: &DaoManager, table: &Table, rows: &Rows) -> Result<(), IntelError> {
    for dao in rows.iter() {
        delete_record_from_table(dm, table, &dao)?;
    }
    Ok(())
}

fn delete_record_from_table(
    dm: &DaoManager,
    table: &Table,
    record: &Dao,
) -> Result<(), IntelError> {
    let mut params: Vec<&Value> = vec![];
    let mut sql = String::from("DELETE FROM ");
    sql += &format!("{} ", table.complete_name());
    sql += "WHERE ";
    let pk = table.get_primary_column_names();
    for (i, col) in pk.iter().enumerate() {
        if i > 0 {
            sql += "AND ";
        }
        let pk_value = record
            .get_value(&col.name)
            .expect("must have primary column values");
        sql += &format!("{} = ${}", col.name, i + 1);
        params.push(pk_value);
    }
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    dm.execute_sql_with_return(&sql, &params)?;
    Ok(())
}

fn save_indirect(
    dm: &DaoManager,
    tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    _indirect_tabs: &Vec<(TableName, Tab)>,
    indirect_records: &Vec<(TableName, TableName, RecordAction, Rows)>,
) -> Result<(), IntelError> {
    for (indirect_tablename, via_tablename, record_action, rows) in indirect_records {
        let indirect_table = table_intel::get_table(indirect_tablename, tables)
            .expect("indirect table should exist");
        let linker_table =
            table_intel::get_table(via_tablename, tables).expect("via table should exists");
        match record_action {
            RecordAction::Unlink => {
                unlink_from_indirect_table(
                    dm,
                    tables,
                    main_table,
                    main_record,
                    indirect_table,
                    linker_table,
                    rows,
                )?;
            }
            RecordAction::LinkNew => {
                link_new_for_indirect_table(
                    dm,
                    tables,
                    main_table,
                    main_record,
                    indirect_table,
                    linker_table,
                    rows,
                )?;
            }
            RecordAction::LinkExisting => {
                link_existing_for_indirect_table(
                    dm,
                    tables,
                    main_table,
                    main_record,
                    indirect_table,
                    linker_table,
                    rows,
                )?;
            }
            _ => {
                println!("unexpected action {:?}", record_action);
            }
        }
    }
    Ok(())
}

/// delete the entry from the linker table
fn unlink_from_indirect_table(
    dm: &DaoManager,
    _tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    indirect_table: &Table,
    linker_table: &Table,
    rows: &Rows,
) -> Result<(), IntelError> {
    for indirect_record in rows.iter() {
        let linker_record = create_linker_record(
            main_table,
            main_record,
            linker_table,
            indirect_table,
            &indirect_record,
        )?;
        delete_record_from_table(dm, linker_table, &linker_record)?;
    }
    Ok(())
}

/// create an entry to the indirect table
/// and create an entry into the linker table
fn link_new_for_indirect_table(
    dm: &DaoManager,
    _tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    indirect_table: &Table,
    linker_table: &Table,
    rows: &Rows,
) -> Result<(), IntelError> {
    for indirect_record in rows.iter() {
        let indirect_record = insert_record_to_table(dm, indirect_table, &indirect_record)?;
        let linker_record = create_linker_record(
            main_table,
            main_record,
            linker_table,
            indirect_table,
            &indirect_record,
        )?;
        insert_record_to_linker_table(dm, linker_table, &linker_record)?;
    }
    Ok(())
}

/// create a record in linker table using the primary key of main and indirect record
fn create_linker_record(
    main_table: &Table,
    main_record: &Dao,
    linker_table: &Table,
    indirect_table: &Table,
    indirect_record: &Dao,
) -> Result<Dao, IntelError> {
    let main_fk_pair = linker_table.get_local_foreign_columns_pair_to_table(&main_table.name);
    let indirect_fk_pair =
        linker_table.get_local_foreign_columns_pair_to_table(&indirect_table.name);
    assert_eq!(main_fk_pair.len(), 1);
    assert_eq!(indirect_fk_pair.len(), 1);
    let (main_linker_local, main_linker_refferred) = main_fk_pair[0];
    let (indirect_linker_local, indirect_linker_refferred) = indirect_fk_pair[0];
    let main_pk_value = main_record
        .get_value(&main_linker_refferred.name)
        .expect("must have a value");
    let indirect_pk_value = indirect_record
        .get_value(&indirect_linker_refferred.name)
        .expect("must have a value");
    let mut linker_record = Dao::new();
    linker_record.insert_value(&main_linker_local.name, main_pk_value);
    linker_record.insert_value(&indirect_linker_local.name, indirect_pk_value);
    Ok(linker_record)
}

/// create an entry to the linker table
/// linking existing record from the indirect table
fn link_existing_for_indirect_table(
    dm: &DaoManager,
    _tables: &Vec<Table>,
    main_table: &Table,
    main_record: &Dao,
    indirect_table: &Table,
    linker_table: &Table,
    rows: &Rows,
) -> Result<(), IntelError> {
    for indirect_record in rows.iter() {
        let linker_record = create_linker_record(
            main_table,
            main_record,
            linker_table,
            indirect_table,
            &indirect_record,
        )?;
        insert_record_to_linker_table(dm, linker_table, &linker_record)?;
    }
    Ok(())
}

/// triggered by the main tab
fn update_records_in_table(
    dm: &DaoManager,
    main_table: &Table,
    rows: &Rows,
) -> Result<Vec<Dao>, IntelError> {
    let mut records = vec![];
    for record in rows.iter() {
        let updated_record = update_record_in_table(dm, main_table, &record)?;
        records.push(updated_record);
    }
    Ok(records)
}

fn update_record_in_table(
    dm: &DaoManager,
    main_table: &Table,
    record: &Dao,
) -> Result<Dao, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("UPDATE {} ", table_name.complete_name());
    let columns = main_table.get_non_primary_columns();
    sql += "SET ";
    for (i, col) in columns.iter().enumerate() {
        let col_sql_type = col.get_sql_type();
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("{} = ${}", col.name.name, i + 1);
        // casting for ts_vector example: film.fulltext tsvector
        // when retrieving: film.fulltext::text
        // when writing data: film.fulltext = $1::ts_vector
        // the _cast is not used however, since we need to cast back to its original type
        if let Some(_cast) = col.cast_as() {
            sql += &format!("::{}", col_sql_type.name());
        }
        if col_sql_type.is_array_type() {
            sql += &format!("::{}", col_sql_type.name());
        }
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
    let bparams: Vec<&Value> = params.iter().collect();
    dm.execute_sql_with_one_return(&sql, &bparams)
}

/// insert rows all at once in one query
fn insert_rows_to_table(dm: &DaoManager, table: &Table, rows: &Rows) -> Result<Rows, IntelError> {
    let table_name = &table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &table.get_non_primary_columns();
    sql += "(";
    for (i, col) in columns.iter().enumerate() {
        if are_all_nil(&col.name.name, rows) && col.is_not_null() && col.has_generated_default() {
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
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("${} ", params.len() + 1);
                let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
                params.push(casted_value);
            }
        }
    }
    sql += ") RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    let bparams: Vec<&Value> = params.iter().collect();
    let rows = dm.execute_sql_with_return(&sql, &bparams)?;
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
    dm: &DaoManager,
    main_table: &Table,
    rows: &Rows,
) -> Result<Vec<Dao>, IntelError> {
    let mut records = vec![];
    for dao in rows.iter() {
        let updated_record = insert_record_to_table(dm, main_table, &dao)?;
        records.push(updated_record);
    }
    Ok(records)
}

fn insert_record_to_table(
    dm: &DaoManager,
    main_table: &Table,
    record: &Dao,
) -> Result<Dao, DbError> {
    let table_name = &main_table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &main_table.get_non_primary_columns();
    sql += "(";
    for (i, col) in columns.iter().enumerate() {
        let value: Option<&Value> = record.get_value(&col.name.name);
        if let Some(ref value) = value {
            if value.is_nil() && col.is_not_null() && col.has_generated_default() {
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("{} ", col.name.name);
            }
        }
    }
    sql += ") ";
    sql += "VALUES (";
    for (i, col) in columns.iter().enumerate() {
        let value = record.get_value(&col.name.name);
        if let Some(ref value) = value {
            if value.is_nil() && col.is_not_null() && col.has_generated_default() {
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("${} ", params.len() + 1);
                let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
                params.push(casted_value);
            }
        }
    }
    sql += ") RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    let bparams: Vec<&Value> = params.iter().collect();
    dm.execute_sql_with_one_return(&sql, &bparams)
}

fn insert_record_to_linker_table(
    dm: &DaoManager,
    linker_table: &Table,
    record: &Dao,
) -> Result<Dao, DbError> {
    let table_name = &linker_table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", table_name.complete_name());
    let columns = &linker_table.get_primary_columns();
    sql += "(";
    for (i, col) in columns.iter().enumerate() {
        let value = record.get_value(&col.name.name);
        if let Some(value) = value {
            if value.is_nil() && col.is_not_null() && col.has_generated_default() {
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("{} ", col.name.name);
            }
        }
    }
    sql += ") ";
    sql += "VALUES (";
    for (i, col) in columns.iter().enumerate() {
        let value = record.get_value(&col.name.name);
        if let Some(value) = value {
            if value.is_nil() && col.is_not_null() && col.has_generated_default() {
            } else {
                if i > 0 {
                    sql += ", ";
                }
                sql += &format!("${} ", params.len() + 1);
                let casted_value = rustorm::common::cast_type(&value, &col.get_sql_type());
                params.push(casted_value);
            }
        }
    }
    sql += ") RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    let bparams: Vec<&Value> = params.iter().collect();
    dm.execute_sql_with_one_return(&sql, &bparams)
}

/// Warning: This only works for postgresql 9.5 and up
/// TODO: make the database trait tell which version is in used
/// use appropriate query for depending on which features are supported
fn upsert_one_one_record_to_table(
    dm: &DaoManager,
    main_table: &Table,
    main_record: &Dao,
    one_one_table: &Table,
    one_one_record: &Dao,
) -> Result<Dao, DbError> {
    let local_referred_pair =
        one_one_table.get_local_foreign_columns_pair_to_table(&main_table.name);

    let mut one_one_record = one_one_record.clone();

    for (one_one_pk, main_pk_name) in local_referred_pair.iter() {
        let main_pk_value = main_record
            .get_value(&main_pk_name.name)
            .expect("should have value");
        one_one_record.insert_value(&one_one_pk.name, main_pk_value);
    }

    let one_one_table_name = &one_one_table.name;
    let mut params = vec![];
    let mut sql = format!("INSERT INTO {} ", one_one_table_name.complete_name());
    let one_one_columns = &one_one_table.columns;
    sql += "(";

    for (i, one_col) in one_one_columns.iter().enumerate() {
        let value = one_one_record.get_value(&one_col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        if value.is_nil() && one_col.is_not_null() && one_col.has_generated_default() {
        } else {
            if i > 0 {
                sql += ", ";
            }
            sql += &format!("{} ", one_col.name.name);
        }
    }
    sql += ") ";
    sql += "VALUES (";

    for (i, one_col) in one_one_columns.iter().enumerate() {
        let value = one_one_record.get_value(&one_col.name.name);
        assert!(value.is_some());
        let value = value.unwrap();
        if value.is_nil() && one_col.is_not_null() && one_col.has_generated_default() {
        } else {
            if i > 0 {
                sql += ", ";
            }
            sql += &format!("${} ", params.len() + 1);
            let casted_value = rustorm::common::cast_type(&value, &one_col.get_sql_type());
            params.push(casted_value);
        }
    }
    sql += ") ";
    let one_one_primary_columns = &one_one_table.get_primary_columns();
    sql += "ON CONFLICT (";

    for (i, one_one_pk) in one_one_primary_columns.iter().enumerate() {
        if i == 0 {
            ;
        } else {
            sql += ", ";
        }
        sql += &format!("{} ", one_one_pk.name.name);
    }
    sql += ") ";
    sql += "DO UPDATE ";
    sql += "SET ";

    for (i, one_col) in one_one_columns.iter().enumerate() {
        if i > 0 {
            sql += ", ";
        }
        sql += &format!("{} = ${}", one_col.name.name, params.len() + 1);
        let value = one_one_record
            .get_value(&one_col.name.name)
            .expect("must have value");
        let casted_value = rustorm::common::cast_type(&value, &one_col.get_sql_type());
        params.push(casted_value);
    }
    sql += " ";

    for (i, (one_one_pk, main_pk_name)) in local_referred_pair.iter().enumerate() {
        if i == 0 {
            sql += "WHERE ";
        } else {
            sql += "AND ";
        }
        sql += &format!(
            "{}.{} = ${} ",
            one_one_table.name.name,
            one_one_pk.name,
            params.len() + 1
        );
        let main_pk = main_table.get_column(main_pk_name).expect("should exist");
        let pk_value = main_record.get_value(&main_pk.name.name);
        assert!(pk_value.is_some());
        let pk_value = pk_value.unwrap();
        let casted_pk_value = rustorm::common::cast_type(&pk_value, &main_pk.get_sql_type());
        params.push(casted_pk_value);
    }
    sql += "RETURNING *";
    println!("sql: {}", sql);
    println!("params: {:?}", params);
    let bparams: Vec<&Value> = params.iter().collect();
    dm.execute_sql_with_one_return(&sql, &bparams)
}
