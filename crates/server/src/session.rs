use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
};
use diwata_intel::{
    cache,
    Context,
    TableName,
    Window,
};
use rustorm::{
    DaoManager,
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
    let db_url = if is_login_required {
        global::get_role_db_url()?
    } else {
        global::get_db_url()?
    };
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&active_em, &db_url)?;
    let tables = cache_pool.get_cached_tables(&active_em, &db_url)?;
    Ok(Context {
        em: active_em,
        dm: active_dm,
        tables: to_hashmap_tables(tables),
        windows: to_hashmap_windows(windows),
    })
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
