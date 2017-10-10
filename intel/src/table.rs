use rustorm::TableName;
use column::Column;
use column::TableKey;

pub struct Table {
    pub name: TableName,

    /// the parent table of this table when inheriting (>= postgresql 9.3)
    pub parent_table: Option<TableName>,

    /// what are the other table that inherits this
    pub sub_table: Vec<TableName>,

    /// comment of this table
    pub comment: Option<String>,

    /// columns of this table
    pub columns: Vec<Column>,

    /// views can also be generated
    pub is_view: bool,

    pub table_key: Vec<TableKey>,

}


impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}
