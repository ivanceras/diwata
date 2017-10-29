use rustorm::TableName;
use tab::Tab;
use rustorm::DbError;
use rustorm::table::SchemaContent;
use rustorm::EntityManager;
use rustorm::Table;
use table_intel::TableIntel;
use table_intel::IndirectTable;

#[derive(Debug)]
pub struct Window {
    /// maps to main table name
    pub name: String,
    /// maps to table comment
    pub description: Option<String>,
    /// group name where this window comes from
    /// maps to table schema
    pub group: Option<String>,
    /// corresponds to the main table 
    main_tab: Tab,
    /// this record is linked 1:1 to this record
    /// and the table that contains that record
    /// is owned in this window and edited here
    one_one_tabs: Vec<Tab>,
    /// the tabs that refers to the selected record
    /// 1:M
    has_many_tabs: Vec<Tab>,
    /// an indirect connection to this record
    /// must have an option to remove/show from the list
    /// async loaded?
    indirect_tabs: Vec<Tab>,
}

impl Window{
    
    fn from_tables(main_table: &Table, one_one: &Vec<&Table>, has_one: &Vec<&Table>,
                   has_many: &Vec<&Table>, indirect: &Vec<IndirectTable>, all_tables: &Vec<Table>) -> Self  {
        let main_tab:Tab = Tab::from_table(main_table, all_tables); 
        let one_one_tabs:Vec<Tab> = one_one.iter().map(|t|Tab::from_table(t, all_tables)).collect();
        //let has_one_tabs:Vec<Tab> = has_one.iter().map(|t|Tab::from_table(t, all_tables)).collect();
        let has_many_tabs:Vec<Tab> = has_many.iter().map(|t|Tab::from_table(t, all_tables)).collect();
        let indirect_tabs:Vec<Tab> = indirect.iter().map(|t|Tab::from_table(t.indirect_table, all_tables)).collect();
        Window{
            name: main_tab.name.to_string(),
            description: main_tab.description.to_owned(),
            group: main_tab.table_name.schema.to_owned(), 
            main_tab,
            one_one_tabs,
            has_many_tabs,
            indirect_tabs
        }
    }
}

#[derive(Debug)]
pub struct WindowName{
    pub name: String,
    pub table_name: TableName,
}


#[derive(Debug)]
pub struct GroupedWindow{
    group: String,
    window_names: Vec<WindowName>
}





/// get all the schema content and convert to grouped window
/// for displaying as a list in the client side
fn get_grouped_windows(em: &EntityManager) -> Result<Vec<GroupedWindow>, DbError> {
    let schema_content: Vec<SchemaContent> = em.get_grouped_tables()?;
    let mut grouped_windows: Vec<GroupedWindow> = Vec::with_capacity(schema_content.len()); 
    for sc in schema_content{
        let mut window_names = Vec::with_capacity(sc.tablenames.len() + sc.views.len());
        for table_name in sc.tablenames.iter().chain(sc.views.iter()){
            window_names.push(WindowName{
                name: table_name.name.to_string(),
                table_name: table_name.to_owned(),
            })
        }
        grouped_windows.push(
            GroupedWindow{
                group: sc.schema.to_string(),
                window_names: window_names
            });
    }
    Ok(grouped_windows)
}


/// extract all the tables and create a window object for each that can
/// be a window, cache them for later use, so as not to keeping redoing 
/// analytical and calculations
fn get_all_windows(em: &EntityManager) -> Result<Vec<Window>, DbError> {
    let tables = em.get_all_tables()?;
    let mut all_windows = Vec::with_capacity(tables.len());
    for table in &tables{
        let table_intel = TableIntel(table);
        if table_intel.is_window(&tables){
            let one_one_tables:Vec<&Table> = table_intel.get_one_one_tables(&tables);
            let has_one_tables:Vec<&Table> = table_intel.get_has_one_tables(&tables);
            let has_many_tables:Vec<&Table> = table_intel.get_has_many_tables(&tables);
            let indirect_tables:Vec<IndirectTable> = table_intel.get_indirect_tables(&tables);
            println!("window: {}", table.name.name);
            let window = Window::from_tables(&table, &one_one_tables, &has_one_tables, 
                                             &has_many_tables, &indirect_tables, &tables);
            all_windows.push(window);
        }
    }
    Ok(all_windows)
}

fn get_window<'t>(table_name: &TableName, windows: &'t Vec<Window>) -> Option<&'t Window> {
    windows.iter()
        .find(|w|w.main_tab.table_name == *table_name)
}

#[cfg(test)]
mod tests{
    use super::*;
    use rustorm::Pool;

    #[test]
    fn all_windows(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v6";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let windows = get_all_windows(&em);
        assert!(windows.is_ok());
        let windows = windows.unwrap();
        assert_eq!(windows.len(), 12);
    }

    #[test]
    fn product_window(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v6";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let windows = get_all_windows(&em);
        assert!(windows.is_ok());
        let windows = windows.unwrap();
        let product = TableName::from("bazaar.product");
        let product_window = get_window(&product, &windows);
        assert!(product_window.is_some());
        let win = product_window.unwrap();

        assert_eq!(win.one_one_tabs.len(), 1);
        assert_eq!(win.one_one_tabs[0].table_name.name, "product_availability");

        assert_eq!(win.has_many_tabs.len(), 0);

        assert_eq!(win.indirect_tabs.len(), 3);
        assert_eq!(win.indirect_tabs[0].table_name.name, "category");
        assert_eq!(win.indirect_tabs[1].table_name.name, "photo");
        assert_eq!(win.indirect_tabs[2].table_name.name, "review");

    }

    #[test]
    fn user_window(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v6";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let windows = get_all_windows(&em);
        assert!(windows.is_ok());
        let windows = windows.unwrap();
        let table = TableName::from("bazaar.users");
        let window = get_window(&table, &windows);
        assert!(window.is_some());
        let win = window.unwrap();
        assert_eq!(win.one_one_tabs.len(), 1);
        assert_eq!(win.one_one_tabs[0].table_name.name, "user_location");

        assert_eq!(win.has_many_tabs.len(), 4);
        assert_eq!(win.has_many_tabs[0].table_name.name, "api_key");
        assert_eq!(win.has_many_tabs[1].table_name.name, "product");
        assert_eq!(win.has_many_tabs[2].table_name.name, "settings");
        assert_eq!(win.has_many_tabs[3].table_name.name, "user_info");

        assert_eq!(win.indirect_tabs.len(), 1);
        assert_eq!(win.indirect_tabs[0].table_name.name, "review");
    }

    #[test]
    fn grouped_windows(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v6";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let grouped_windows = get_grouped_windows(&em);
        assert!(grouped_windows.is_ok());
        let grouped_windows = grouped_windows.unwrap();
        println!("grouped windows: {:#?}", grouped_windows);
        assert_eq!(grouped_windows.len(), 4);
    }
}
