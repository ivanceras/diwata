use crate::app::field_view::FieldView;

use data_table::DataRow;

pub struct RowView {
    fields: Vec<FieldView>,
}

impl RowView {
    pub fn new(data_rows: DataRow) -> Self {
        RowView {
            fields: data_rows.into_iter().map(FieldView::new).collect(),
        }
    }
}
