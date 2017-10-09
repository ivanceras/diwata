

pub struct TableName{
    name: String,
    schema: Option<String>,
}

impl TableName{
    fn from(s: &str) -> Self {
        if s.contains(".") {
            let splinters = s.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 parts");
            let schema = splinters[0].to_owned();
            let table = splinters[1].to_owned();
            Table {
                schema: Some(schema),
                name: table,
            }
        } else {
            Table {
                schema: None,
                name: s.to_owned(),
            }
        }
    }

    pub fn complete_name(&self) -> String {
        match self.schema {
            Some(ref schema) => format!("{}.{}", schema, self.name),
            None => self.name.to_owned(),
        }
    }
}


pub struct Table {
    pub name: TableName,

    /// the parent table of this table when inheriting (>= postgresql 9.3)
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub parent_table: Option<String>,

    /// what are the other table that inherits this
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub sub_table: Vec<String>,

    /// comment of this table
    pub comment: Option<String>,

    /// columns of this table
    pub columns: Vec<Column>,

    /// views can also be generated
    pub is_view: bool,

    /// estimated row count if any
    pub estimated_row_count: Option<usize>,
}
