use data_table::DataColumn;

pub struct ColumnView {
    data_column: DataColumn,
}

impl ColumnView {
    pub fn new(data_column: DataColumn) -> Self {
        ColumnView { data_column }
    }
}
