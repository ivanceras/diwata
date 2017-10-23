use rustorm::TableName;
use tab::Tab;

pub struct WindowName{
    pub name: String,
    pub table_name: TableName,
}


pub struct GroupedWindow{
    group: String,
    window_name: Vec<WindowName>
}

pub struct OrganizedWindow(pub Vec<GroupedWindow>);


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
    /// this record contains a record from this table
    /// but has it's own window tab/editor
    /// and that record may have been referenced by some other
    /// record too, M:1
    has_one_tabs: Vec<Tab>,
    /// the tabs that refers to the selected record
    /// 1:M
    has_many_tab: Vec<Tab>,
    /// an indirect connection to this record
    /// must have an option to remove/show from the list
    /// async loaded?
    indirect_tabs: Vec<Tab>,
}
