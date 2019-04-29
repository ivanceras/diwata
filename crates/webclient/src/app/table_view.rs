use crate::app::{column_view::ColumnView, row_view::RowView};
use data_table::{DataColumn, DataTable};
use diwata_intel::{Field, Tab};

pub struct TableView {
    pub columns: Vec<ColumnView>,
    pub rows: Vec<RowView>,
}

impl TableView {
    pub fn from_tab(tab: Tab) -> Self {
        TableView {
            columns: tab
                .fields
                .iter()
                .map(Self::convert_field_to_column_view)
                .collect(),
            rows: vec![],
        }
    }

    pub fn from_data_table(data_table: DataTable) -> Self {
        TableView {
            columns: data_table
                .columns
                .into_iter()
                .map(ColumnView::new)
                .collect(),
            rows: data_table.rows.into_iter().map(RowView::new).collect(),
        }
    }

    fn convert_field_to_column_view(field: &Field) -> ColumnView {
        let data_column = DataColumn {
            name: field.name.clone(),
            description: field.description.clone(),
            tags: vec![],
            data_type: field.get_data_type().clone(),
        };
        ColumnView::new(data_column)
    }
}
