use rustorm::ColumnName;
use rustorm::Record;
use rustorm::Rows;
use rustorm::TableName;

#[derive(Debug, Serialize)]
pub struct RecordDetail {
    pub record: Record,
    pub one_ones: Vec<(TableName, Option<Record>)>,
    pub has_many: Vec<(TableName, Rows)>,
    // (linker_tablename, indirect_tablename, records)
    pub indirect: Vec<(TableName, TableName, Rows)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RecordAction {
    Unlink,
    LinkExisting,
    LinkNew,
    Edited,    // only used in the main record
    CreateNew, // only used in the main record
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordChangeset {
    pub record: Record,
    pub action: RecordAction,
    pub one_ones: Vec<(TableName, Option<Record>)>,
    pub has_many: Vec<(TableName, RecordAction, Rows)>,
    pub indirect: Vec<(TableName, TableName, RecordAction, Rows)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveContainer {
    pub for_insert: (TableName, Rows),
    pub for_update: (TableName, Rows),
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
        Condition {
            left: column_name,
            right: value,
        }
    }
}

/// a limited filter structure which is used for the simple usecase of the client
/// all conditions are AND together, and the operator depends on the data type of the column name
/// String will be ILIKE '%?'
/// Date will be in between
/// number will text_cast then ilike
pub struct Filter {
    pub conditions: Vec<Condition>,
}

impl Filter {
    pub fn from_str(s: &str) -> Self {
        let splinters: Vec<&str> = s.split("&").collect();
        let mut conditions = vec![];
        for splinter in splinters.iter() {
            let cond = Condition::from_str(splinter);
            conditions.push(cond);
        }
        Filter { conditions }
    }
}

pub struct Order {
    pub column_name: ColumnName,
    pub direction: Direction,
}

impl Order {
    fn from_str(s: &str) -> Self {
        let splinters: Vec<&str> = s.split(".").collect();
        let len = splinters.len();
        let mut cols = splinters.clone();
        let dir = cols.split_off(len - 1);
        let direction = if dir.len() == 1 {
            let dir = dir[0];
            match dir {
                "asc" => Some(Direction::Asc),
                "desc" => Some(Direction::Desc),
                _ => None,
            }
        } else {
            None
        };
        let column = cols.join(".");
        let column_name = ColumnName::from(&column);
        match direction {
            Some(direction) => Order {
                column_name: column_name,
                direction: direction,
            },
            None => Order {
                column_name: ColumnName::from(&splinters.join(".")),
                direction: Direction::Asc,
            },
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

pub struct Sort {
    pub orders: Vec<Order>,
}

impl Sort {
    pub fn from_str(s: &str) -> Self {
        let splinters: Vec<&str> = s.split(",").collect();
        let mut orders = vec![];
        for splinter in splinters.iter() {
            let order = Order::from_str(splinter);
            orders.push(order);
        }
        Sort { orders }
    }
}

#[test]
fn test_record_changeset() {
    extern crate serde_json;
    use dao::Dao;

    let input = r#"
{
  "record": {
    "city": {
      "Text": "Akishima"
    },
    "city_id": {
      "Int": 10
    },
    "country_id": {
      "Int": 50
    },
    "last_update": {
      "Timestamp": "2006-02-15T17:45:25Z"
    }
  },
  "action": "Edited",
  "one_ones": [],
  "has_many": [
    [
      {
        "name": "address",
        "schema": "public",
        "alias": null
      },
      "Edited",
      {
        "columns": [
          "address_id",
          "address",
          "address2",
          "district",
          "postal_code",
          "phone",
          "last_update",
          "city_id"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "address",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "address_id",
          "address",
          "address2",
          "district",
          "postal_code",
          "phone",
          "last_update",
          "city_id"
        ],
        "data": []
      }
    ]
  ],
  "indirect": []
}
    "#;
    let mut dao = Dao::new();
    dao.insert("city", "Akishima");
    let changeset = RecordChangeset {
        record: Record::from(&dao),
        action: RecordAction::Edited,
        one_ones: vec![],
        has_many: vec![],
        indirect: vec![],
    };
    let changeset_json = serde_json::to_string(&changeset).unwrap();
    println!("changeset json: {}", changeset_json);
    let result: Result<RecordChangeset, _> = serde_json::from_str(input);
    println!("result: {:#?}", result);
    assert!(result.is_ok());
}
