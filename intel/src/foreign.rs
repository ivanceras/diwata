use rustorm::TableName;
use rustorm::ColumnName;


pub struct Foreign {
    pub table: TableName,
    pub column: ColumnName,
}
