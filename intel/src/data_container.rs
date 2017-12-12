use rustorm::TableName;
use rustorm::Rows;
use rustorm::Record;
use rustorm::ColumnName;
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


#[derive(Debug, Serialize)]
pub struct Lookup{
    /// the lookup data in main table
    main_lookup: Vec<(TableName, ColumnName, DropdownInfo, Rows)>, 
    /// for the 1:1 lookup data
    one_one_lookup: Vec<(TableName, ColumnName, DropdownInfo, Rows)>, 
    /// the lookup used in the has_many tab details
    hasmany_lookup: Vec<(TableName, ColumnName, DropdownInfo, Rows)>,
    /// the lookup used in the indirect tab details
    indirect_lookup: Vec<(TableName, ColumnName, DropdownInfo, Rows)>
}

