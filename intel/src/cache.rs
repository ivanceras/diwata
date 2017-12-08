use rustorm::Table;
use window::Window;
use window;
use rustorm::EntityManager;
use std::collections::BTreeMap;
use error::IntelError;
use std::sync::{Arc,Mutex};


lazy_static!{
    pub static ref CACHE_POOL: Arc<Mutex<CachePool>> = {
        Arc::new(Mutex::new(CachePool::new()))
    };
}




/// contains an atlas of cache, one for each String key
/// puposedly for DB_URL, there will
/// be a separate cache for each DB_URL
/// that tries to connect to the service
pub struct CachePool(BTreeMap<String,Cache>);

impl CachePool{

    fn new() -> Self {
        CachePool(BTreeMap::new())
    }

    /// reset all cache content including cache from other DB_URLs
    fn reset_all(&mut self) {
        self.0.clear();
    }

    /// clear the cache on this DB_URL
    fn clear(&mut self, db_url: &str) -> Option<Cache> {
        self.0.remove(db_url)
    }

    fn ensure_cache(&mut self, db_url: &str) {
        if !self.0.contains_key(db_url){
            self.0.insert(db_url.to_string(), Cache::new());
        }
    }

    fn has_table_cache(&self, db_url: &str) -> bool {
        match self.0.get(db_url){
            Some(cache) => cache.has_table_cache(),
            None => false
        }
    }

    fn has_window_cache(&self, db_url: &str) -> bool {
        match self.0.get(db_url){
            Some(cache) => cache.has_window_cache(),
            None => false
        }
    }

    pub fn get_cached_tables(&mut self, em: &EntityManager, db_url: &str) -> Result<Vec<Table>, IntelError> {
        self.ensure_cache(db_url);
        if self.has_table_cache(db_url){
           let cache = self.0.get(db_url); 
           match cache{
               Some(cache) => {
                   match cache.tables{
                       Some(ref tables) => {
                           println!("TABLE CACHE HIT!");
                           Ok(tables.clone())
                       }
                       None => Err(IntelError::CacheServiceError)
                   }
                }
               None => {
                   Err(IntelError::CacheServiceError)
               }
            }
        }
        else{
            // do a caching and try again
            println!("Performing a TABLE caching and trying again");
            self.perform_table_caching(em, db_url)?;
            self.get_cached_tables(em, db_url)
        }
    }

    pub fn get_cached_windows(&mut self, em: &EntityManager, db_url: &str) -> Result<Vec<Window>, IntelError> {
        self.ensure_cache(db_url);
        if self.has_window_cache(db_url){
           let cache = self.0.get(db_url); 
           match cache{
               Some(cache) => {
                   match cache.windows{
                       Some(ref windows) => {
                           println!("WINDOW CACHE HIT!");
                           Ok(windows.clone())
                       }
                       None => Err(IntelError::CacheServiceError)
                   }
                }
               None => {
                   Err(IntelError::CacheServiceError)
               }
            }
        }
        else{
            // do a caching and try again
            println!("Performing a WINDOW caching and trying again");
            self.perform_window_caching(em, db_url)?;
            self.get_cached_windows(em, db_url)
        }
    }

    fn perform_table_caching(&mut self, em: &EntityManager, db_url: &str) -> Result<(), IntelError> {
        let cache = self.0.get_mut(db_url);
        match cache{
            Some(cache) => cache.perform_table_caching(em),
            None => Err(IntelError::CacheServiceError) 
        }
    }

    fn perform_window_caching(&mut self, em: &EntityManager, db_url: &str) -> Result<(), IntelError> {
        let cache = self.0.get_mut(db_url);
        match cache{
            Some(cache) => cache.perform_window_caching(em),
            None => Err(IntelError::CacheServiceError) 
        }
    }


}

/// items cached, unique for each db_url connection
pub struct Cache {
    /// windows extraction is an expensive operation and doesn't change very often
    /// None indicates, that nothing is cached yet, empty can be indicated as cached
    pub windows: Option<Vec<Window>>,
    /// tables extraction is an expensive operation and doesn't change very often
    pub tables: Option<Vec<Table>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            windows: None,
            tables: None,
        }
    }

    fn has_table_cache(&self) -> bool {
        self.tables.is_some()
    }

    fn has_window_cache(&self) -> bool {
        self.windows.is_some()
    }

    fn perform_table_caching(&mut self, em: &EntityManager) -> Result<(), IntelError>{
        println!("----> ACTUAL TABLE CACHING");
        let tables = em.get_all_tables()?;
        self.tables = Some(tables);
        Ok(())
    }

    fn perform_window_caching(&mut self, em: &EntityManager) -> Result<(), IntelError>{
        println!("----> ACTUAL WINDOW CACHING");
        match self.tables{
            Some(ref tables) => {
                self.windows = Some(window::derive_all_windows(&tables));
                Ok(())
            }
            None => {
                self.perform_table_caching(em)?;
                self.perform_window_caching(em)?;
                Ok(())
            }
        }
    }

}
