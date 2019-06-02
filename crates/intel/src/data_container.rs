use crate::Window;
use either::Either;
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

/// Holds the result for a sql query
/// If there are multiple records
/// it will in Either::Left rows,
/// and if there is only 1 record, happens
/// when the where clause specify a Primary_key = id
/// Then that record is retrieved and it's additional details as well.
/// such as 1:1 records and related records in has_many and indirect table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub window: Option<Window>,
    pub record: Either<Rows, RecordDetail>,
}

impl QueryResult {
    /// When there are multiple results of records
    pub fn with_rows(window: Option<&Window>, rows: Rows) -> Self {
        QueryResult {
            window: window.map(Clone::clone),
            record: Either::Left(rows),
        }
    }

    /// When there is only 1 record
    pub fn with_record_detail(
        window: Option<&Window>,
        record_detail: RecordDetail,
    ) -> Self {
        QueryResult {
            window: window.map(Clone::clone),
            record: Either::Right(record_detail),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDetail {
    pub window: Window,
    pub record: Dao,
    pub one_ones: Vec<(TableName, Option<Dao>)>,
    pub has_many: Vec<(TableName, Rows)>,
    // NOTE: indirect vec(linker_tablename, indirect_tablename, records)
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentifierDisplay {
    pub columns: Vec<ColumnName>,
    pub pk: Vec<ColumnName>,
    pub separator: Option<String>,
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
