use rustorm::TableName;
use rustorm::ColumnName;
use rustorm::table::ForeignKey;

use field::Field;
use rustorm::Table;
use rustorm::Column;
use table_intel;

#[derive(Debug, Serialize, Clone)]
pub struct Tab {
    pub name: String,
    pub description: Option<String>,
    pub table_name: TableName,
    /// simple fields, the lookup fields are not included
    /// in these
    pub fields: Vec<Field>,
    pub is_view: bool,
}

impl Tab {
    pub fn from_table(table: &Table, tables: &Vec<Table>) -> Self {
        let fields = Tab::derive_fields(table, tables);
        Tab {
            name: table.name.name.to_string(),
            description: table.comment.to_owned(),
            table_name: table.name.to_owned(),
            fields: fields,
            is_view: table.is_view,
        }
    }

    fn derive_fields(table: &Table, tables: &Vec<Table>) -> Vec<Field> {
        let mut fields = Vec::with_capacity(table.columns.len());
        fields.extend(Tab::derive_simple_fields(table));
        fields.extend(Tab::derive_foreign_fields(table, tables));
        fields
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
}
