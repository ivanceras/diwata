use error::ServiceError;
use intel::Window;
use intel::cache;
use rustorm::EntityManager;
use rustorm::RecordManager;
use rustorm::Table;
use global;

pub struct Context {
    pub em: EntityManager,
    pub dm: RecordManager,
    pub tables: Vec<Table>,
    pub windows: Vec<Window>,
}

impl Context {
    pub fn create() -> Result<Self, ServiceError> {
        let dm = global::get_pool_dm()?;
        let em = global::get_pool_em()?;
        let db_url = &global::get_db_url()?;
        let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        Ok(Context {
            em,
            dm,
            tables,
            windows,
        })
    }
}
