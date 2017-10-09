use rustorm::TableName;
use rustorm::ColumnName;
use types::SqlType;
use foreign::Foreign;

pub struct Column {
    pub table: Option<TableName>,
    pub name: ColumnName,
    pub data_type: SqlType,
    pub is_primary: bool,
    pub is_unique: bool,
    pub default: Option<String>,
    pub comment: Option<String>,
    pub not_null: bool,
    pub foreign: Option<Foreign>,
    /// determines if the column is inherited from the parent table
    pub is_inherited: bool,
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}
