use error::ServiceError;
use intel::Window;
use intel::cache;
use rustorm::EntityManager;
use rustorm::DaoManager;
use rustorm::Table;
use global;
use credentials::Credentials;

pub struct Context {
    pub em: EntityManager,
    pub dm: DaoManager,
    pub tables: Vec<Table>,
    pub windows: Vec<Window>,
}

impl Context {
    pub fn create(credentials: Result<Credentials, ServiceError>) -> Result<Self, ServiceError> {
        let dm = global::get_pool_dm()?;
        let em = global::get_pool_em()?;
        let is_login_required = global::is_login_required()?;
        if is_login_required {
            set_session_credentials(&credentials?, &em)?;
        }

        let active_em = if is_login_required {
            global::get_pool_session_em()?
        }else{
            em
        };
        let active_dm = if is_login_required {
            global::get_pool_session_dm()?
        }else{
            dm
        };
        let db_url = if is_login_required{
            global::get_role_db_url()?
        }else{
            global::get_db_url()?
        };
        let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&active_em, &db_url)?;
        let tables = cache_pool.get_cached_tables(&active_em, &db_url)?;
        Ok(Context {
            em: active_em,
            dm: active_dm,
            tables,
            windows,
        })
    }
}

/// set the session user for the database connection
/// call this in every data request to ensure appropriate
/// database previlege is imposed for the next database queries
fn set_session_credentials(credentials: &Credentials, em: &EntityManager) -> Result<(), ServiceError> {
    println!("------------->>>> SETTING SESSION CREDENTIALS");
    em.set_session_user(&credentials.username)?;
    let role = em.get_role(&credentials.username)?;
    match role{
        Some(role) => {
            let current_db_url = global::get_db_url()?;
            println!("current_db_url {}", current_db_url);
            let session_db_url = global::recreate_db_url(&credentials.username, Some(&credentials.password), &current_db_url)?;
            global::set_session_db_url(&session_db_url)?;
            println!("session_db_url: {}", session_db_url);
            let role_db_url = global::recreate_db_url(&role.role_name, None, &current_db_url)?;
            println!("role_db_url: {}", role_db_url);
            global::set_role_db_url(&role_db_url)?;
            Ok(())
        }
        None => {
            panic!("No role for {}", credentials.username);
        }
    }
}
