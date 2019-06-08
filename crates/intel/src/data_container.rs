use crate::{
    window::GroupedWindow,
    Window,
};

use rustorm::{
    ColumnName,
    Dao,
    Rows,
    TableName,
    Value,
};
use serde::{
    Deserialize,
    Serialize,
};

/// an arranged value with respect  to the fields arrangement in tab
pub type DataRow = Vec<Value>;

#[derive(Default, Serialize, Deserialize)]
pub struct AppData {
    pub grouped_window: Vec<GroupedWindow>,
    pub windows: Vec<Window>,
    pub window_data: Vec<WindowData>,
}

/// Page a collection of rows
/// also shows the total records from the table source
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Page {
    /// page number
    pub page: usize,
    /// rows on this page
    pub rows: Vec<DataRow>,
}

impl Page {
    fn from_rows(rows: Rows) -> Self {
        Page {
            page: 1,
            rows: rows.data,
        }
    }
}

/// Convert the dao into a vec of value
/// TODO: ensure the alignment of column and data
fn data_row_from_dao(dao: Dao) -> DataRow {
    let mut values = vec![];
    for (_k, v) in dao.0.into_iter() {
        values.push(v);
    }
    values
}

/// Contains all the data for a window
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WindowData {
    /// The sql query used to obtain this data,
    pub sql_query: Option<String>,
    /// pages can be scrolled to and fro
    /// and sometimes unloaded for performance puposed
    pub main_tab_data: Vec<Page>,
    pub main_tab_total_rows: usize,
    /// current page of the main tab rows
    pub main_tab_current_page: usize,
    /// Contains the main tab record detail when in detailed view
    pub record_detail: Option<RecordDetail>,
    pub one_one_tab_data: Vec<Option<DataRow>>,
    /// Vector of pages for each has_many_tab
    pub has_many_tab_data: Vec<Vec<Page>>,
    /// current page of the has_many_tabs
    pub has_many_tab_current_page: Vec<usize>,
    pub has_many_tab_total_rows: Vec<usize>,
    /// Vector of pages for each indirect_tab
    pub indirect_tab_data: Vec<Vec<Page>>,
    pub indirect_tab_current_page: Vec<usize>,
    pub indirect_tab_total_rows: Vec<usize>,

    /// Frozen data for each of this tab
    pub main_tab_frozen_data: FrozenData,
    pub has_many_tab_frozen_data: Vec<FrozenData>,
    pub indirect_tab_frozen_data: Vec<FrozenData>,
}

impl WindowData {
    pub fn from_rows(rows: Rows) -> Self {
        if rows.count.is_none() {
            println!("there is no row count ");
        }
        WindowData {
            main_tab_total_rows: rows.count.unwrap_or(0),
            main_tab_current_page: 1,
            main_tab_data: vec![Page::from_rows(rows)],
            ..Default::default()
        }
    }

    pub fn add_main_data_page(&mut self, rows: Rows){
        sauron::log!("Added {} rows", rows.data.len());
        self.main_tab_data.push(Page::from_rows(rows));
    }

    pub fn set_record_detail(&mut self, record_detail: RecordDetail) {
        self.record_detail = Some(record_detail.clone());
        self.one_one_tab_data = record_detail.one_ones.into_iter().fold(
            vec![],
            |mut acc, (_table_name, row)| {
                acc.push(row.map(data_row_from_dao));
                acc
            },
        );
        self.has_many_tab_current_page = record_detail.has_many.iter().fold(
            vec![],
            |mut acc, (_table_name, _rows)| {
                acc.push(1);
                acc
            },
        );

        self.has_many_tab_total_rows = record_detail.has_many.iter().fold(
            vec![],
            |mut acc, (_table_name, rows)| {
                acc.push(rows.count.unwrap_or(0));
                acc
            },
        );
        self.has_many_tab_data = record_detail.has_many.into_iter().fold(
            vec![],
            |mut acc, (_table_name, rows)| {
                acc.push(vec![Page::from_rows(rows)]);
                acc
            },
        );
        self.indirect_tab_total_rows = record_detail.indirect.iter().fold(
            vec![],
            |mut acc, (_linker, _table_name, rows)| {
                acc.push(rows.count.unwrap_or(0));
                acc
            },
        );

        self.indirect_tab_current_page = record_detail.indirect.iter().fold(
            vec![],
            |mut acc, (_linker, _table_name, _rows)| {
                acc.push(1);
                acc
            },
        );
        self.indirect_tab_data = record_detail.indirect.into_iter().fold(
            vec![],
            |mut acc, (_linker, _table_name, rows)| {
                acc.push(vec![Page::from_rows(rows)]);
                acc
            },
        );
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FrozenData {
    pub frozen_rows: Vec<usize>,
    pub frozen_columns: Vec<usize>,
}

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
    pub rows: Rows,
}

impl QueryResult {
    /// When there are multiple results of records
    pub fn with_rows(window: Option<&Window>, rows: Rows) -> Self {
        QueryResult {
            window: window.map(Clone::clone),
            rows,
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
