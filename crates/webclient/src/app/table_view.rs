use crate::app::{column_view::ColumnView, row_view::RowView};
use data_table::{DataColumn, DataTable};
use diwata_intel::{Field, Tab};
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

use crate::app::column_view;

#[derive(Clone)]
pub enum Msg {
    ColumnMsg(usize, column_view::Msg),
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
}

impl Component<Msg> for TableView {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        main(
            [class("tab")],
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
                section([], []),
            ],
        )
    }
}
