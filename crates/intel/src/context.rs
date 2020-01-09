use crate::{
    window::GroupedWindow,
    Window,
};
use rustorm::{
    Table,
    TableName,
};
use std::collections::HashMap;

pub struct Context {
    pub tables: HashMap<TableName, Table>,
    pub windows: HashMap<TableName, Window>,
    pub grouped_window: Vec<GroupedWindow>,
}

impl Context {
    /// get the window matching the table name and its schema
    pub fn get_window(&self, table_name: &TableName) -> Option<&Window> {
        self.windows.get(table_name)
    }

    /// find the window match the complete name first,
    /// if it can't be found, match only the name without the schame
    pub fn find_window(&self, table_name: &TableName) -> Option<&Window> {
        self.get_window(table_name).or(self.windows.iter().find_map(
            |(k, v)| {
                if k.name == table_name.name {
                    Some(v)
                } else {
                    None
                }
            },
        ))
    }

    pub fn get_table(&self, table_name: &TableName) -> Option<&Table> {
        self.tables.get(table_name)
    }
}
