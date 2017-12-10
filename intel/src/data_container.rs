use rustorm::TableName;
use rustorm::Rows;
use rustorm::Record;
use rustorm::ColumnName;

#[derive(Debug, Serialize)]
pub struct RecordDetail {
    pub record: Record,
    pub one_ones: Vec<(TableName, Option<Record>)>,
    pub has_many: Vec<(TableName, Rows)>,
    pub indirect: Vec<(TableName, Rows)>,
}

pub struct Lookup{
    /// main table may contain 1:1 tables
    main_lookup: Vec<(TableName, ColumnName, Rows)>, 
    hasmany_lookup: Vec<(TableName, ColumnName, Rows)>,
    indirect_lookup: Vec<(TableName, ColumnName, Rows)>
}

