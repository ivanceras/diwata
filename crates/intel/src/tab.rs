use rustorm::{
    ColumnName,
    TableName,
};

use crate::{
    data_container::{
        DropdownInfo,
        IdentifierDisplay,
    },
    field::Field,
};
use rustorm::{
    Column,
    Table,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug,PartialEq,  Serialize, Clone, Deserialize)]
pub struct Tab {
    pub name: String,
    pub description: Option<String>,
    pub table_name: TableName,
    /// simple fields, the lookup fields are not included
    /// in these
    pub fields: Vec<Field>,
    pub is_view: bool,
    pub display: Option<IdentifierDisplay>,
}

/// an indirect connection to this record
/// must have an option to remove/show from the list
/// async loaded?
#[derive(Debug,PartialEq,  Serialize, Deserialize, Clone)]
pub struct IndirectTab {
    pub linker: TableName,
    pub tab: Tab,
}

impl IndirectTab {
    pub fn new(linker: TableName, tab: Tab) -> Self {
        IndirectTab { linker, tab }
    }
}

impl Tab {
    pub fn from_table(
        table: &Table,
        name: Option<String>,
        tables: &[Table],
    ) -> Self {
        let fields = Self::derive_fields(table, tables);
        let display = Self::derive_display(table);
        let tab_name = match name {
            Some(name) => name,
            None => table.name.name.to_string(),
        };
        Tab {
            name: tab_name,
            description: table.comment.to_owned(),
            table_name: table.name.to_owned(),
            fields,
            is_view: table.is_view,
            display,
        }
    }

    /// The arrangement of fields are changed from the original arrangement in the table columns
    /// causing a misalignment in the display
    fn derive_fields(table: &Table, _tables: &[Table]) -> Vec<Field> {
        table
            .columns
            .iter()
            .map(|col| Field::from_column(table, col))
            .collect()
    }

    pub fn derive_dropdowninfo(table: &Table) -> Option<DropdownInfo> {
        match Self::derive_display(table) {
            Some(display) => {
                Some(DropdownInfo {
                    source: table.name.clone(),
                    display,
                })
            }
            None => None,
        }
    }

    /// an identifier column alone by itself
    #[allow(clippy::if_same_then_else)]
    fn is_identifier_column(table: &Table, column: &Column) -> bool {
        let table_name = &table.name.name;
        let column_name = &column.name.name;
        if column_name == "name" {
            true
        } else if column_name == table_name {
            true
        } else if *column_name == format!("{}_name", table_name) {
            true
        } else if column_name == "title" {
            true
        } else if table_name == "user" || table_name == "users" {
            column_name == "name"
                || column_name == "username"
                || column_name == "email"
        } else {
            false
        }
    }

    // TODO: also make a function to do derive_image_display to detect
    // images that are displayeable
    fn derive_display(table: &Table) -> Option<IdentifierDisplay> {
        let columns = &table.columns;
        let pk: Vec<ColumnName> = table
            .get_primary_column_names()
            .iter()
            .map(|ref column| (**column).to_owned())
            .collect();

        let non_pk_columns: Vec<ColumnName> = table
            .get_non_primary_columns()
            .iter()
            .map(|column| column.name.to_owned())
            .collect();

        let single_identifier = columns
            .iter()
            .find(|column| Self::is_identifier_column(&table, column));
        if let Some(single_identifier) = single_identifier {
            return Some(IdentifierDisplay {
                columns: vec![single_identifier.name.clone()],
                separator: None,
                pk: pk.clone(),
            });
        }
        // if there is only 1 non primary column use it as the identifier column
        else if non_pk_columns.len() == 1 {
            return Some(IdentifierDisplay {
                columns: non_pk_columns,
                separator: None,
                pk: pk.clone(),
            });
        }
        // match the column name regardless of the table name
        else {
            let maybe_firstname = columns.iter().find(|column| {
                let column_name = &column.name.name;
                *column_name == "first_name" || *column_name == "firstname"
            });

            let maybe_lastname = columns.iter().find(|column| {
                let column_name = &column.name.name;
                *column_name == "last_name" || *column_name == "lastname"
            });
            if let Some(lastname) = maybe_lastname {
                if let Some(firstname) = maybe_firstname {
                    return Some(IdentifierDisplay {
                        columns: vec![
                            lastname.name.clone(),
                            firstname.name.clone(),
                        ],
                        separator: Some(", ".into()),
                        pk: pk.clone(),
                    });
                }
            }
        }

        // always have idenfier display
        Some(IdentifierDisplay {
            columns: vec![],
            separator: None,
            pk: pk.clone(),
        })
    }

    pub fn get_display_columns(&self) -> Vec<&ColumnName> {
        match self.display {
            Some(ref display) => {
                display.columns.iter().map(|ref column| *column).collect()
            }
            None => vec![],
        }
    }

    pub fn has_column_name(&self, column_name: &ColumnName) -> bool {
        self.fields
            .iter()
            .any(|field| field.has_column_name(column_name))
    }
}

pub fn find_tab<'a>(
    tabs: &'a [Tab],
    table_name: &TableName,
) -> Option<&'a Tab> {
    tabs.iter().find(|tab| tab.table_name == *table_name)
}
