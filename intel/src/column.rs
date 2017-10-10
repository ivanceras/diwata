use rustorm::TableName;
use rustorm::ColumnName;
use types::SqlType;
use foreign::Foreign;

pub struct Column {
    pub table: Option<TableName>,
    pub name: ColumnName,
    pub comment: Option<String>,
    pub specification: ColumnSpecification,
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

pub struct PrimaryKey{
    name: Option<String>,
    columns: Vec<ColumnName>,
}
pub struct UniqueKey{
    name: Option<String>,
    columns: Vec<ColumnName>,
}

pub struct ForeignKey{
    name: Option<String>,
    table: TableName,
    column: ColumnName,
}

pub struct Key{
    name: String,
    columns: Vec<ColumnName>,
}

pub enum TableKey {
    PrimaryKey(PrimaryKey),
    UniqueKey(UniqueKey),
    Key(Key),
    ForeignKey(ForeignKey),
}

pub enum ColumnConstraint {
    NotNull,
    DefaultValue(Literal),
    AutoIncrement,
}

pub struct ColumnSpecification {
    pub sql_type: SqlType,
    pub constraints: Vec<ColumnConstraint>,
}


pub enum Literal {
    Null,
    Integer(i64),
    UuidV4, // pg: uuid_generate_v4();
    String(String),
    Blob(Vec<u8>),
    CurrentTime, // pg: now()
    CurrentDate, //pg: today()
    CurrentTimestamp, // pg: now()
}

impl From<i64> for Literal {
    fn from(i: i64) -> Self {
        Literal::Integer(i)
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

impl<'a> From<&'a str> for Literal {
    fn from(s: &'a str) -> Self {
        Literal::String(String::from(s))
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match *self {
            Literal::Null => "NULL".to_string(),
            Literal::Integer(ref i) => format!("{}", i),
            Literal::UuidV4 => format!("uuid_generate_v4()"), //FIXME: finalize this
            Literal::String(ref s) => format!("'{}'", s),
            Literal::Blob(ref bv) => format!(
                "{}",
                bv.iter()
                    .map(|v| format!("{:x}", v))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Literal::CurrentTime => "CURRENT_TIME".to_string(),
            Literal::CurrentDate => "CURRENT_DATE".to_string(),
            Literal::CurrentTimestamp => "CURRENT_TIMESTAMP".to_string(),
        }
    }
}


