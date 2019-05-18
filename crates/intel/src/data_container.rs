use rustorm::{
    ColumnName,
    Dao,
    Rows,
    TableName,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize)]
pub struct RecordDetail {
    pub record: Dao,
    pub one_ones: Vec<(TableName, Option<Dao>)>,
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
    pub record: Dao,
    pub action: RecordAction,
    pub one_ones: Vec<(TableName, Option<Dao>)>,
    pub has_many: Vec<(TableName, RecordAction, Rows)>,
    // vec ( table, via linker, action, rows )
    pub indirect: Vec<(TableName, TableName, RecordAction, Rows)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveContainer {
    pub for_insert: (TableName, Rows),
    pub for_update: (TableName, Rows),
}
/// the dropdown data and the description on
/// how will it be displayed as defined in IdentifierDisplay
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropdownInfo {
    // source table of this records
    pub source: TableName,
    // derived from the table tabs describing how the record is
    // displayed on compact space
    pub display: IdentifierDisplay,
}

/// lookup for same table are the same regardless of which field they are referred
#[derive(Debug, Deserialize, Serialize)]
pub struct Lookup(pub Vec<(TableName, Rows)>);

/// the displayable column name, serves as identifier to human vision
/// this would be name, title, first_name - lastname
#[derive(Debug, Serialize,Deserialize,  Clone)]
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
impl From<&str> for Condition {
    //TODO: verify if the column is really a column of the involved tables otherwise SQL injection
    //is possible
    fn from(s: &str) -> Self {
        let splinters: Vec<&str> = s.split('=').collect();
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
impl From<&str> for Filter {
    fn from(s: &str) -> Self {
        let splinters: Vec<&str> = s.split('&').collect();
        let mut conditions = vec![];
        for splinter in splinters.iter() {
            let cond = Condition::from(*splinter);
            conditions.push(cond);
        }
        Filter { conditions }
    }
}

pub struct Order {
    pub column_name: ColumnName,
    pub direction: Direction,
}

impl From<&str> for Order {
    fn from(s: &str) -> Self {
        let splinters: Vec<&str> = s.split('.').collect();
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
            Some(direction) => {
                Order {
                    column_name,
                    direction,
                }
            }
            None => {
                Order {
                    column_name: ColumnName::from(&splinters.join(".")),
                    direction: Direction::Asc,
                }
            }
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
impl From<&str> for Sort {
    fn from(s: &str) -> Self {
        let splinters: Vec<&str> = s.split(',').collect();
        let mut orders = vec![];
        for splinter in splinters.iter() {
            let order: Order = (*splinter).into();
            orders.push(order);
        }
        Sort { orders }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_changeset() {
        use rustorm::Dao;

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
            record: dao,
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

    #[test]
    fn test_record_changeset2() {
        extern crate serde_json;
        use rustorm::Dao;

        let input = r#"
{
  "record": {
    "description": {
      "Text": "A Fanciful Documentary of a Frisbee And a Lumberjack who must Chase a Monkey in A Shark Tank"
    },
    "film_id": {
      "Int": 4
    },
    "fulltext": {
      "Text": "'affair':1 'chase':14 'documentari':5 'fanci':4 'frisbe':8 'lumberjack':11 'monkey':16 'must':13 'prejudic':2 'shark':19 'tank':20"
    },
    "language_id": {
      "Int": 1
    },
    "last_update": {
      "Timestamp": "2007-09-11T01:46:03Z"
    },
    "length": {
      "Smallint": 117
    },
    "original_language_id": "Nil",
    "rating": {
      "Text": "G"
    },
    "release_year": {
      "Smallint": 2006
    },
    "rental_duration": {
      "Smallint": 5
    },
    "rental_rate": {
      "BigDecimal": "2.99"
    },
    "replacement_cost": {
      "BigDecimal": "26.99"
    },
    "title": {
      "Text": "AFFAIR PREJUDICE"
    }
  },
  "action": "Edited",
  "one_ones": [],
  "has_many": [
    [
      {
        "name": "inventory",
        "schema": "public"
      },
      "Edited",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "inventory",
        "schema": "public"
      },
      "LinkNew",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "inventory",
        "schema": "public",
        "alias": null
      },
      "Unlink",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": []
      }
    ]
  ],
  "indirect": [
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "actor_id",
          "first_name",
          "last_name",
          "last_update"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "category_id",
          "name",
          "last_update"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public",
        "alias": null
      },
      "LinkExisting",
      {
        "columns": [
          "actor_id"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public"
      },
      "LinkExisting",
      {
        "columns": [
          "category_id"
        ],
        "data": [
          [
            {
              "Int": 3
            }
          ]
        ]
      }
    ],
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public"
      },
      "Unlink",
      {
        "columns": [
          "actor_id",
          "first_name",
          "last_name",
          "last_update"
        ],
        "data": []
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public",
        "alias": null
      },
      "Unlink",
      {
        "columns": [
          "category_id",
          "name",
          "last_update"
        ],
        "data": []
      }
    ]
  ]
}
    "#;
        let mut dao = Dao::new();
        dao.insert("city", "Akishima");
        let changeset = RecordChangeset {
            record: dao,
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

    #[test]
    fn test_record_changeset3() {
        use rustorm::Dao;

        let input = r#"
{
  "record": {
    "description": {
      "Text": "A Fanciful Documentary of a Frisbee And a Lumberjack who must Chase a Monkey in A Shark Tank"
    },
    "film_id": {
      "Int": 4
    },
    "fulltext": {
      "Text": "'affair':1 'chase':14 'documentari':5 'fanci':4 'frisbe':8 'lumberjack':11 'monkey':16 'must':13 'prejudic':2 'shark':19 'tank':20"
    },
    "language_id": {
      "Int": 1
    },
    "last_update": {
      "Timestamp": "2007-09-11T01:46:03Z"
    },
    "length": {
      "Smallint": 117
    },
    "original_language_id": "Nil",
    "rating": {
      "Text": "G"
    },
    "release_year": {
      "Smallint": 2006
    },
    "rental_duration": {
      "Smallint": 5
    },
    "rental_rate": {
      "BigDecimal": "2.99"
    },
    "replacement_cost": {
      "BigDecimal": "26.99"
    },
    "title": {
      "Text": "AFFAIR PREJUDICE"
    }
  },
  "action": "Edited",
  "one_ones": [],
  "has_many": [
    [
      {
        "name": "inventory",
        "schema": "public",
        "alias": null
      },
      "Edited",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": [],
        "count": null
      }
    ],
    [
      {
        "name": "inventory",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": [],
        "count": null
      }
    ],
    [
      {
        "name": "inventory",
        "schema": "public",
        "alias": null
      },
      "Unlink",
      {
        "columns": [
          "inventory_id",
          "last_update",
          "film_id",
          "store_id"
        ],
        "data": [],
        "count": null
      }
    ]
  ],
  "indirect": [
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "actor_id",
          "first_name",
          "last_name",
          "last_update"
        ],
        "data": [],
        "count": null
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public",
        "alias": null
      },
      "LinkNew",
      {
        "columns": [
          "category_id",
          "name",
          "last_update"
        ],
        "data": [],
        "count": null
      }
    ],
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public",
        "alias": null
      },
      "LinkExisting",
      {
        "columns": [
          "actor_id"
        ],
        "data": [
          []
        ],
        "count": null
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public",
        "alias": null
      },
      "LinkExisting",
      {
        "columns": [
          "category_id"
        ],
        "data": [
          [
            {
              "Int": 3
            }
          ]
        ],
        "count": null
      }
    ],
    [
      {
        "name": "actor",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_actor",
        "schema": "public",
        "alias": null
      },
      "Unlink",
      {
        "columns": [
          "actor_id",
          "first_name",
          "last_name",
          "last_update"
        ],
        "data": [],
        "count": null
      }
    ],
    [
      {
        "name": "category",
        "schema": "public",
        "alias": null
      },
      {
        "name": "film_category",
        "schema": "public",
        "alias": null
      },
      "Unlink",
      {
        "columns": [
          "category_id",
          "name",
          "last_update"
        ],
        "data": [],
        "count": null
      }
    ]
  ]
}
    "#;
        let mut dao = Dao::new();
        dao.insert("city", "Akishima");
        let changeset = RecordChangeset {
            record: dao,
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
}
