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

/// the dropdown data and the description on
/// how will it be displayed as defined in IdentifierDisplay
#[derive(Debug, Serialize, Clone)]
pub struct DropdownInfo {
    // source table of this records
    pub source: TableName,
    // derived from the table tabs describing how the record is
    // displayed on compact space
    pub display: IdentifierDisplay,
}

/// lookup for same table are the same regardless of which field they are referred
#[derive(Debug, Serialize)]
pub struct Lookup(pub Vec<(TableName, Rows)>);

/// the displayable column name, serves as identifier to human vision
/// this would be name, title, first_name - lastname
#[derive(Debug, Serialize, Clone)]
pub struct IdentifierDisplay {
    pub columns: Vec<ColumnName>,
    pub pk: Vec<ColumnName>,
    pub separator: Option<String>,
}
