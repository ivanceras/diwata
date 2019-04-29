use crate::app::field_view::FieldView;

use data_table::DataRow;

pub struct DetailView {
    fields: Vec<FieldView>,
}

impl DetailView {
    pub fn new(data_rows: DataRow) -> Self {
        DetailView {
            fields: data_rows.into_iter().map(FieldView::new).collect(),
        }
    }
}
