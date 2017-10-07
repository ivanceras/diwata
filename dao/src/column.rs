


pub struct Column {
    pub name: String,
    pub table: Option<String>,
    pub alias: Option<String>,
}


pub trait ToColumns {
    /// extract the columns from struct
    fn to_columns() -> Vec<Column>;
}
