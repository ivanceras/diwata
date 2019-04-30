use data_table::{DataRow, Value};

/// Page a collection of rows
/// also shows the total records from the table source
#[derive(Default)]
pub struct Page {
    /// page number
    page: usize,
    /// rows on this page
    rows: Vec<DataRow>,
    total_records: usize,
}

/// Contains all the data for a window
#[derive(Default)]
pub struct WindowData {
    /// pages can be scrolled to and fro
    /// and sometimes unloaded for performance puposed
    main_data: Vec<Page>,
    /// Vector of pages for each has_many_tab
    has_many_data: Vec<Vec<Page>>,
    /// Vector of pages for each indirect_tab
    indirect_data: Vec<Vec<Page>>,

    /// Frozen data for each of this tab
    pub main_frozen_data: FrozenData,
    pub has_many_frozen_data: FrozenData,
    pub indirect_frozen_data: FrozenData,
}

impl WindowData {
    pub fn main_rows(self) -> Vec<DataRow> {
        self.main_data
            .into_iter()
            .flat_map(|page| page.rows)
            .collect()
    }
}

#[derive(Default, Clone)]
pub struct FrozenData {
    pub frozen_rows: Vec<usize>,
    pub frozen_columns: Vec<usize>,
}

fn make_sample_frozen_data() -> FrozenData {
    FrozenData {
        frozen_rows: vec![0, 1, 5],
        frozen_columns: vec![0, 1],
    }
}

pub fn make_sample_window_data() -> WindowData {
    WindowData {
        main_data: vec![make_sample_page()],
        has_many_data: vec![vec![make_sample_page()]],
        indirect_data: vec![vec![make_sample_page()]],
        main_frozen_data: make_sample_frozen_data(),
        has_many_frozen_data: make_sample_frozen_data(),
        indirect_frozen_data: make_sample_frozen_data(),
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
    (0..40).into_iter().map(|n| make_sample_row(n)).collect()
}
pub fn make_sample_row(row: usize) -> Vec<Value> {
    (0..25)
        .into_iter()
        .map(|n| Value::Text(format!("Row{}-Value{}", row, n)))
        .collect()
}
