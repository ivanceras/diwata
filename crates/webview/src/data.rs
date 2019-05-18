use data_table::{DataRow, Value};
use diwata_intel::Rows;
use diwata_intel::data_container::RecordDetail;
use diwata_intel::Dao;

/// Page a collection of rows
/// also shows the total records from the table source
#[derive(Default)]
pub struct Page {
    /// page number
    pub page: usize,
    /// rows on this page
    pub rows: Vec<DataRow>,
    pub total_records: usize,
}

impl Page {
    fn from_rows(rows: Rows) -> Self {
        Page {
            page: 1,
            rows: rows.data,
            total_records: rows.count.unwrap_or(0),
        }
    }

    fn from_dao(dao: Dao) -> Self {
        Page {
            page: 1,
            rows: vec![data_row_from_dao(dao)],
            total_records: 1,
        }
    }
}

/// Convert the dao into a vec of value
/// TODO: ensure the alignment of column and data
fn data_row_from_dao(dao: Dao) -> DataRow {
    let mut values = vec![];
    for (k, v) in dao.0.into_iter(){
        values.push(v);
    }
    values
}

/// Contains all the data for a window
#[derive(Default)]
pub struct WindowData {
    /// The sql query used to obtain this data,
    pub sql_query: Option<String>,
    /// pages can be scrolled to and fro
    /// and sometimes unloaded for performance puposed
    pub main_tab_data: Vec<Page>,
    pub one_one_tab_data: Vec<DataRow>,
    /// Vector of pages for each has_many_tab
    pub has_many_tab_data: Vec<Vec<Page>>,
    /// Vector of pages for each indirect_tab
    pub indirect_tab_data: Vec<Vec<Page>>,

    /// Frozen data for each of this tab
    pub main_tab_frozen_data: FrozenData,
    pub has_many_tab_frozen_data: Vec<FrozenData>,
    pub indirect_tab_frozen_data: Vec<FrozenData>,
}

impl WindowData {
    pub fn from_rows(rows: Rows) -> Self {
        WindowData {
            main_tab_data: vec![Page::from_rows(rows)],
            ..Default::default()
        }
    }

    pub fn from_record_detail(record_detail: RecordDetail) -> Self {
        WindowData{
            main_tab_data: vec![Page::from_dao(record_detail.record)],
            //TODO: also set the related records here
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
pub struct FrozenData {
    pub frozen_rows: Vec<usize>,
    pub frozen_columns: Vec<usize>,
}

fn make_sample_frozen_data() -> FrozenData {
    FrozenData {
        frozen_rows: vec![0, 1],
        frozen_columns: vec![0, 1],
    }
}

pub fn make_sample_window_data() -> WindowData {
    WindowData {
        sql_query: Some("select * from placeholder".to_string()),
        main_tab_data: vec![make_sample_page()],
        one_one_tab_data: vec![make_sample_row(0), make_sample_row(1)],
        has_many_tab_data: vec![vec![make_sample_page()]],
        indirect_tab_data: vec![vec![make_sample_page()]],
        main_tab_frozen_data: make_sample_frozen_data(),
        has_many_tab_frozen_data: vec![make_sample_frozen_data()],
        indirect_tab_frozen_data: vec![make_sample_frozen_data()],
    }
}

pub fn make_sample_page() -> Page {
    Page {
        page: 0,
        rows: make_sample_rows(),
        total_records: 100,
    }
}

pub fn make_sample_rows() -> Vec<DataRow> {
    (0..40).map(make_sample_row).collect()
}
pub fn make_sample_row(row: usize) -> Vec<Value> {
    (0..25)
        .map(|n| Value::Text(format!("Row{}-Value{}", row, n)))
        .collect()
}
