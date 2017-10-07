
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub schema: Option<String>,
    pub alias: Option<String>,
}

pub trait ToTable {
    /// extract the table info
    fn to_table(&self) -> Table;
}
