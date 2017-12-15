use rustorm::TableName;
use rustorm::Rows;
use rustorm::Record;
use tab::IdentifierDisplay;

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

