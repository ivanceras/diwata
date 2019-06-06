use crate::{
    tab::{
        IndirectTab,
        Tab,
    },
    table_intel::{
        IndirectTable,
        TableIntel,
    },
};
use rustorm::{
    ColumnName,
    Table,
    TableName,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Window {
    /// maps to main table name
    pub name: String,

    /// maps to table comment
    pub description: Option<String>,

    /// group name where this window comes from
    /// maps to table schema
    pub group: Option<String>,

    /// corresponds to the main table
    pub main_tab: Tab,

    /// table names that is referred by fields from the main table
    /// the first page of it is retrieved
    pub has_one_tabs: Vec<Tab>,

    /// this record is linked 1:1 to this record
    /// and the table that contains that record
    /// is owned in this window and edited here
    pub one_one_tabs: Vec<Tab>,

    /// the tabs that refers to the selected record
    /// 1:M
    pub has_many_tabs: Vec<Tab>,

    pub indirect_tabs: Vec<IndirectTab>,

    pub is_view: bool,
}

impl Window {
    fn from_tables(
        main_table: &Table,
        one_one: &[&Table],
        has_one: &[&Table],
        has_many: &[&Table],
        indirect: &[IndirectTable],
        all_tables: &[Table],
    ) -> Self {
        let main_tab: Tab = Tab::from_table(main_table, None, all_tables);
        let one_one_tabs: Vec<Tab> = one_one
            .iter()
            .map(|t| Tab::from_table(t, None, all_tables))
            .collect();
        let has_one_tabs: Vec<Tab> = has_one
            .iter()
            .map(|t| Tab::from_table(t, None, all_tables))
            .collect();
        let has_many_tabs: Vec<Tab> = has_many
            .iter()
            .map(|t| Tab::from_table(t, None, all_tables))
            .collect();
        let is_view = main_tab.is_view;

        let indirect_tabs: Vec<IndirectTab> = indirect
            .iter()
            .map(|t| {
                let has_repeat =
                    has_repeating_tab(&t.indirect_table.name, indirect);
                let tab_name = if has_repeat {
                    Some(format!(
                        "{} (via {})",
                        t.indirect_table.name.name, t.linker.name.name
                    ))
                } else {
                    None
                };
                IndirectTab::new(
                    t.linker.name.clone(),
                    Tab::from_table(t.indirect_table, tab_name, all_tables),
                )
            })
            .collect();
        Window {
            name: main_tab.name.to_string(),
            description: main_tab.description.to_owned(),
            group: main_tab.table_name.schema.to_owned(),
            main_tab,
            has_one_tabs,
            one_one_tabs,
            has_many_tabs,
            indirect_tabs,
            is_view,
        }
    }

    pub fn has_column_name(&self, column_name: &ColumnName) -> bool {
        self.main_tab.has_column_name(column_name)
            || self
                .has_many_tabs
                .iter()
                .any(|tab| tab.has_column_name(column_name))
            || self.indirect_tabs.iter().any(|indirect_tab| {
                indirect_tab.tab.has_column_name(column_name)
            })
    }

    pub fn table_name(&self) -> TableName {
        self.main_tab.table_name.clone()
    }
}

fn has_repeating_tab(
    table_name: &TableName,
    indirect: &[IndirectTable],
) -> bool {
    let mut matched = 0;
    for ind in indirect.iter() {
        if ind.indirect_table.name == *table_name {
            matched += 1;
        }
    }
    matched > 1
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowName {
    pub name: String,
    pub table_name: TableName,
    pub is_view: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedWindow {
    pub group: String,
    pub window_names: Vec<WindowName>,
}

/// extract all the tables and create a window object for each that can
/// be a window, cache them for later use, so as not to keeping redoing
/// analytical and calculations
pub fn derive_all_windows(tables: &[Table]) -> Vec<Window> {
    let mut all_windows = Vec::with_capacity(tables.len());
    for table in tables {
        let table_intel = TableIntel(table);
        if table_intel.is_window(&tables) {
            let one_one_tables: Vec<&Table> =
                table_intel.get_one_one_tables(&tables);
            let has_one_tables: Vec<&Table> =
                table_intel.get_has_one_tables(&tables);
            let has_many_tables: Vec<&Table> =
                table_intel.get_has_many_tables(&tables);
            let indirect_tables: Vec<IndirectTable> =
                table_intel.get_indirect_tables(&tables);
            println!("window: {}", table.name.name);
            let window = Window::from_tables(
                &table,
                &one_one_tables,
                &has_one_tables,
                &has_many_tables,
                &indirect_tables,
                &tables,
            );
            all_windows.push(window);
        }
    }
    all_windows
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustorm::Pool;

    #[test]
    fn all_windows() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let tables = em.get_all_tables().unwrap();
        let windows = derive_all_windows(&tables);
        //assert_eq!(windows.len(), 12); // 12 when not including owned windows
        assert_eq!(windows.len(), 26); // 26 when owned tables can be windows too
    }

    #[test]
    fn product_window() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let tables = em.get_all_tables().unwrap();
        let windows = derive_all_windows(&tables);
        let product = TableName::from("bazaar.product");
        let product_window = get_window(&product, &windows);
        assert!(product_window.is_some());
        let win = product_window.unwrap();

        assert_eq!(win.one_one_tabs.len(), 1);
        assert_eq!(win.one_one_tabs[0].table_name.name, "product_availability");

        assert_eq!(win.has_many_tabs.len(), 1);

        assert_eq!(win.indirect_tabs.len(), 3);
        assert_eq!(win.indirect_tabs[0].tab.table_name.name, "category");
        assert_eq!(win.indirect_tabs[1].tab.table_name.name, "photo");
        assert_eq!(win.indirect_tabs[2].tab.table_name.name, "review");
    }

    #[test]
    fn user_window() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let tables = em.get_all_tables().unwrap();
        let windows = derive_all_windows(&tables);
        let table = TableName::from("bazaar.users");
        let window = get_window(&table, &windows);
        assert!(window.is_some());
        let win = window.unwrap();
        assert_eq!(win.one_one_tabs.len(), 1);
        assert_eq!(win.one_one_tabs[0].table_name.name, "user_location");

        assert_eq!(win.has_many_tabs.len(), 5);
        assert_eq!(win.has_many_tabs[0].table_name.name, "api_key");
        assert_eq!(win.has_many_tabs[1].table_name.name, "product");
        assert_eq!(win.has_many_tabs[2].table_name.name, "review");
        assert_eq!(win.has_many_tabs[3].table_name.name, "settings");
        assert_eq!(win.has_many_tabs[4].table_name.name, "user_info");

        assert_eq!(win.indirect_tabs.len(), 1);
        assert_eq!(win.indirect_tabs[0].tab.table_name.name, "review");
    }

}
