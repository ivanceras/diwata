use crate::app::{column_view::ColumnView, row_view::RowView};
use data_table::{DataColumn, DataTable};
use diwata_intel::{Field, Tab};
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

use crate::app::{column_view, row_view};
use data_table::DataRow;

#[derive(Clone)]
pub enum Msg {
    ColumnMsg(usize, column_view::Msg),
    RowMsg(usize, row_view::Msg),
}

pub struct TableView {
    pub column_views: Vec<ColumnView>,
    pub row_views: Vec<RowView>,
}

impl TableView {
    pub fn from_tab(tab: Tab) -> Self {
        TableView {
            column_views: tab
                .fields
                .iter()
                .map(Self::convert_field_to_column_view)
                .collect(),
            row_views: vec![],
        }
    }

    pub fn from_data_table(data_table: DataTable) -> Self {
        TableView {
            column_views: data_table
                .columns
                .into_iter()
                .map(ColumnView::new)
                .collect(),
            row_views: data_table.rows.into_iter().map(RowView::new).collect(),
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

    /// replace all the data with a new data row
    pub fn set_data_rows(&mut self, data_row: Vec<DataRow>) {
        self.row_views = data_row.into_iter().map(RowView::new).collect();
    }
}

impl Component<Msg> for TableView {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        main(
            [class("table")],
            [
                header(
                    [class("column_view_names")],
                    self.column_views
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            column
                                .view()
                                .map(move |column_msg| Msg::ColumnMsg(index, column_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                ol(
                    [class("rows")],
                    self.row_views
                        .iter()
                        .enumerate()
                        .map(|(index, row_view)| {
                            row_view
                                .view()
                                .map(move |row_msg| Msg::RowMsg(index, row_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
