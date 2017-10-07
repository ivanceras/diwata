
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub schema: Option<String>,
    pub alias: Option<String>,
}

impl Table {
    pub fn name(&self) -> String {
        if let Some(ref schema) = self.schema {
            format!("{}.{}", schema, self.name)
        } else {
            format!("{}", self.name)
        }
    }
}

pub trait ToTable {
    /// extract the table name from a struct
    fn to_table() -> Table;
}
