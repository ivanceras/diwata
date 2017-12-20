use rustorm::TableName;
use rustorm::ColumnName;
use rustorm::table::ForeignKey;

use field::Field;
use rustorm::Table;
use rustorm::Column;
use table_intel;
use data_container::DropdownInfo;

#[derive(Debug, Serialize, Clone)]
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

/// the displayable column name, serves as identifier to human vision
/// this would be name, title, first_name - lastname
#[derive(Debug, Serialize, Clone)]
pub struct IdentifierDisplay {
    pub columns: Vec<ColumnName>,
    pub pk: Vec<ColumnName>,
    separator: Option<String>,
}

impl Tab {
    pub fn from_table(table: &Table, tables: &Vec<Table>) -> Self {
        let fields = Self::derive_fields(table, tables);
        let display = Self::derive_display(table);
        Tab {
            name: table.name.name.to_string(),
            description: table.comment.to_owned(),
            table_name: table.name.to_owned(),
            fields: fields,
            is_view: table.is_view,
            display,
        }
    }

    fn derive_fields(table: &Table, tables: &Vec<Table>) -> Vec<Field> {
        let mut fields = Vec::with_capacity(table.columns.len());
        fields.extend(Self::derive_simple_fields(table));
        fields.extend(Self::derive_foreign_fields(table, tables));
        fields
    }

    pub fn derive_dropdowninfo(table: &Table) -> Option<DropdownInfo> {
        match Self::derive_display(table) {
            Some(display) => Some(DropdownInfo {
                source: table.name.clone(),
                display,
            }),
            None => None,
        }
    }

    // TODO: also make a function to do derive_image_display to detect
    // images that are displayeable
    fn derive_display(table: &Table) -> Option<IdentifierDisplay> {
        let table_name = &table.name.name;
        let columns = &table.columns;
        let pk: Vec<ColumnName> = table
            .get_primary_column_names()
            .iter()
            .map(|ref column| (**column).to_owned())
            .collect();
        // match for users table common structure
        let display = if table_name == "user" || table_name == "users" {
            let found_column = columns.iter().find(|column| {
                let column_name = &column.name.name;
                *column_name == "username" || *column_name == "email"
            });
            found_column.map(|column| IdentifierDisplay {
                columns: vec![column.name.clone()],
                separator: None,
                pk: pk.clone(),
            })
        }
        // match the column name regardless of the table name
        else {
            let found_column = columns.iter().find(|column| {
                let column_name = &column.name.name;
                *column_name == "name" || *column_name == "title"
            });
            found_column.map(|column| IdentifierDisplay {
                columns: vec![column.name.clone()],
                separator: None,
                pk: pk.clone(),
            })
        };

        // match for person common columns
        display.or_else(|| {
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
                    Some(IdentifierDisplay {
                        columns: vec![lastname.name.clone(), firstname.name.clone()],
                        separator: Some(", ".into()),
                        pk: pk.clone(),
                    })
                } else {
                    None
                }
            } else {
                let same_name = columns.iter().find(|column| {
                    let column_name = &column.name.name;
                    column_name == table_name
                });

                match same_name {
                    Some(column) => Some(IdentifierDisplay {
                        columns: vec![column.name.clone()],
                        separator: None,
                        pk: pk.clone(),
                    }),
                    None => {
                        // use primary key orunique key here
                        let mut columns: Vec<ColumnName> = vec![];
                        let primary_columns = table.get_primary_column_names();
                        for pk in primary_columns {
                            columns.push(pk.to_owned());
                        }
                        Some(IdentifierDisplay {
                            columns,
                            separator: None,
                            pk: pk.clone(),
                        })
                    }
                }
            }
        })
    }

    fn derive_simple_fields(table: &Table) -> Vec<Field> {
        let columns: &Vec<Column> = &table.columns;
        let foreign_column_names: Vec<&ColumnName> = table.get_foreign_column_names();
        let plain_columns: Vec<&Column> = columns
            .iter()
            .filter(|c| !foreign_column_names.contains(&&c.name))
            .collect();
        let mut fields: Vec<Field> = Vec::with_capacity(plain_columns.len());
        for pc in plain_columns {
            let field = Field::from_column(table, pc);
            fields.push(field)
        }
        fields
    }

    fn derive_foreign_fields(table: &Table, all_tables: &Vec<Table>) -> Vec<Field> {
        let foreign_keys: Vec<&ForeignKey> = table.get_foreign_keys();
        let mut fields: Vec<Field> = Vec::with_capacity(foreign_keys.len());
        for fk in foreign_keys {
            let mut columns: Vec<&Column> = Vec::with_capacity(fk.columns.len());
            for fc in &fk.columns {
                if let Some(col) = table.get_column(fc) {
                    columns.push(col);
                }
            }
            let foreign_table = table_intel::get_table(&fk.foreign_table, all_tables);
            if let Some(foreign_table) = foreign_table {
                let field = Field::from_has_one_table(table, &columns, foreign_table);
                fields.push(field);
            }
        }
        fields
    }

    pub fn get_display_columns(&self) -> Vec<&ColumnName> {
        match *&self.display {
            Some(ref display) => display.columns.iter().map(|ref column| *column).collect(),
            None => vec![],
        }
    }
}
