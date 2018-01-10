use rustorm::TableName;
use rustorm::Rows;
use rustorm::Record;
use rustorm::ColumnName;

#[derive(Debug, Serialize)]
pub struct RecordDetail {
    pub record: Record,
    pub one_ones: Vec<(TableName, Option<Record>)>,
    pub has_many: Vec<(TableName, Rows)>,
    // (linker_tablename, indirect_tablename, records)
    pub indirect: Vec<(TableName, TableName, Rows)>,
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

/// a limited condition statement, just needed for the simple usecase
pub struct Condition {
    pub left: ColumnName,
    pub right: String,
}

impl Condition {

    //TODO: verify if the column is really a column of the involved tables otherwise SQL injection
    //is possible
    fn from_str(s: &str) -> Self {
        let splinters: Vec<&str> = s.split("=").collect();
        assert_eq!(splinters.len(), 2);
        let column = splinters[0];
        let value = splinters[1].to_string();
        let column_name = ColumnName::from(column);
        Condition{
            left: column_name,
            right: value
        }
    }
}

/// a limited filter structure which is used for the simple usecase of the client
/// all conditions are AND together, and the operator depends on the data type of the column name
/// String will be ILIKE '%?'
/// Date will be in between
/// number will text_cast then ilike
pub struct Filter{
    pub conditions: Vec<Condition>,
}

impl Filter{

    pub fn from_str(s: &str) -> Self {
        let splinters: Vec<&str> = s.split("&").collect();
        let mut conditions = vec![];
        for splinter in splinters.iter(){
            let cond = Condition::from_str(splinter);
            conditions.push(cond);
        }
        Filter{
            conditions
        }
    }
}
