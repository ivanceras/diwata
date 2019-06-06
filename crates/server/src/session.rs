use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
};
use diwata_intel::{
    cache,
    table_intel::{
        self,
        TableIntel,
    },
    window::{
        GroupedWindow,
        WindowName,
    },
    Context,
    TableName,
    Window,
};
use rustorm::{
    table::SchemaContent,
    EntityManager,
    Table,
};
use std::collections::HashMap;

pub fn create_context(
    credentials: Result<Credentials, ServiceError>,
) -> Result<Context, ServiceError> {
    let dm = global::get_pool_dm()?;
    let em = global::get_pool_em()?;
    let is_login_required = global::is_login_required()?;
    if is_login_required {
        set_session_credentials(&credentials?, &em)?;
    }

    let active_em = if is_login_required {
        global::get_pool_session_em()?
    } else {
        em
    };
    let active_dm = if is_login_required {
        global::get_pool_session_dm()?
    } else {
        dm
    };
    let db_url = global::get_db_url()?;

    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&active_em, &db_url)?;
    let tables = cache_pool.get_cached_tables(&active_em, &db_url)?;
    let grouped_window = get_grouped_windows(&active_em, &tables)?;
    Ok(Context {
        em: active_em,
        dm: active_dm,
        tables: to_hashmap_tables(tables),
        windows: to_hashmap_windows(windows),
        grouped_window,
    })
}

/// get all the schema content and convert to grouped window
/// for displaying as a list in the client side
/// filter out tablenames that are not window
fn get_grouped_windows(
    em: &EntityManager,
    tables: &[Table],
) -> Result<Vec<GroupedWindow>, ServiceError> {
    let schema_content: Vec<SchemaContent> = em.get_grouped_tables()?;
    let mut grouped_windows: Vec<GroupedWindow> =
        Vec::with_capacity(schema_content.len());
    for sc in schema_content {
        let mut window_names =
            Vec::with_capacity(sc.tablenames.len() + sc.views.len());
        for table_name in sc.tablenames.iter().chain(sc.views.iter()) {
            let table = table_intel::get_table(&table_name, tables);
            if let Some(table) = table {
                let table_intel = TableIntel(table);
                if table_intel.is_window(tables) {
                    window_names.push(WindowName {
                        name: table_name.name.to_string(),
                        table_name: table_name.to_owned(),
                        is_view: table.is_view,
                    })
                }
            }
        }
        grouped_windows.push(GroupedWindow {
            group: sc.schema.to_string(),
            window_names,
        });
    }
    Ok(grouped_windows)
}

fn to_hashmap_tables(tables: Vec<Table>) -> HashMap<TableName, Table> {
    let mut hash = HashMap::new();
    for table in tables {
        hash.insert(table.name.clone(), table);
    }
    hash
}

fn to_hashmap_windows(windows: Vec<Window>) -> HashMap<TableName, Window> {
    let mut hash = HashMap::new();
    for win in windows {
        hash.insert(win.main_tab.table_name.clone(), win);
    }
    hash
}

/// set the session user for the database connection
/// call this in every data request to ensure appropriate
/// database previlege is imposed for the next database queries
fn set_session_credentials(
    credentials: &Credentials,
    em: &EntityManager,
) -> Result<(), ServiceError> {
    println!("------------->>>> SETTING SESSION CREDENTIALS");
    em.set_session_user(&credentials.username)?;
    let role = em.get_role(&credentials.username)?;
    match role {
        Some(role) => {
            let current_db_url = global::get_db_url()?;
            println!("current_db_url {}", current_db_url);
            let session_db_url = global::recreate_db_url(
                &credentials.username,
                Some(&credentials.password),
                &current_db_url,
            )?;
            global::set_session_db_url(&session_db_url)?;
            println!("session_db_url: {}", session_db_url);
            let role_db_url = global::recreate_db_url(
                &role.role_name,
                None,
                &current_db_url,
            )?;
            println!("role_db_url: {}", role_db_url);
            global::set_role_db_url(&role_db_url)?;
            Ok(())
        }
        None => {
            Err(ServiceError::GenericError(format!(
                "no role for {}",
                credentials.username
            )))
        }
    }
}
