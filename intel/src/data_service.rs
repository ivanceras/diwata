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

pub struct Filter;


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
    let main_table = table_intel::get_table(main_tablename, tables);
    assert!(main_table.is_some());
    let main_table = main_table.unwrap();
    sql += &format!("FROM {} \n",main_tablename.complete_name());
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
            sql += &format!("   LEFT JOIN {} \n", has1.complete_name());
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
                    sql += &format!("{}.{} = {}.{}\n",main_tablename.name, col.name, has1.name, rcol.name) 
                }
            }
        }
    }
    sql += &format!("LIMIT {}", page_size);
    println!("SQL: {}", sql);
    let result: Result<Rows, DbError> = em.db().execute_sql_with_return(&sql, &[]);
    println!("result: {:?}", result);
    result
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

