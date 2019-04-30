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
    /// Which columns of the rows are to be frozen on the left side of the table
    frozen_rows: Vec<usize>,
    frozen_columns: Vec<usize>,
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
            frozen_rows: vec![],
            frozen_columns: vec![],
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
            frozen_rows: vec![],
            frozen_columns: vec![],
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

    pub fn freeze_rows(&mut self, rows: Vec<usize>) {
        self.frozen_rows = rows;
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        self.frozen_columns = columns.clone();
        self.row_views
            .iter_mut()
            .for_each(|row_view| row_view.freeze_columns(columns.clone()))
    }
}

impl Component<Msg> for TableView {
    fn update(&mut self, _msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        main(
            [class("table")],
            [section(
                [class("rows_and_frozen_columns")],
                [
                    section(
                        [class("frozen_column_names_and_frozen_column_rows")],
                        [
                            header(
                                [class("frozen_column_names")],
                                self.column_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _column)| self.frozen_columns.contains(index))
                                    .map(|(index, column)| {
                                        column.view().map(move |column_msg| {
                                            Msg::ColumnMsg(index, column_msg)
                                        })
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                            // absolutely immovable frozen column and row
                            // can not move in any direction
                            ol(
                                [class("immovable_frozen_columns")],
                                self.row_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _row_view)| self.frozen_rows.contains(index))
                                    .map(|(index, row_view)| {
                                        div(
                                            [class("selector_and_frozen_column_rows")],
                                            [
                                                input([r#type("checkbox")], []),
                                                row_view.view_frozen().map(move |row_msg| {
                                                    Msg::RowMsg(index, row_msg)
                                                }),
                                            ],
                                        )
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                            // can move up and down
                            ol(
                                [class("frozen_columns")],
                                self.row_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _row_view)| !self.frozen_rows.contains(index))
                                    .map(|(index, row_view)| {
                                        // The checkbox selection and the rows of the frozen
                                        // columns
                                        div(
                                            [class("selector_and_frozen_column_rows")],
                                            [
                                                input([r#type("checkbox")], []),
                                                row_view.view_frozen().map(move |row_msg| {
                                                    Msg::RowMsg(index, row_msg)
                                                }),
                                            ],
                                        )
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                        ],
                    ),
                    section(
                        [class("frozen_rows_and_normal_rows")],
                        [
                            // can move left and right
                            header(
                                [class("normal_column_view_names")],
                                self.column_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _column)| !self.frozen_columns.contains(index))
                                    .map(|(index, column)| {
                                        column.view().map(move |column_msg| {
                                            Msg::ColumnMsg(index, column_msg)
                                        })
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                            // can move left and right, but not up and down
                            ol(
                                [class("frozen_rows")],
                                self.row_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _row_view)| self.frozen_rows.contains(&index))
                                    .map(|(index, row_view)| {
                                        row_view
                                            .view()
                                            .map(move |row_msg| Msg::RowMsg(index, row_msg))
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                            // can move: left, right, up, down
                            ol(
                                [class("normal_rows")],
                                self.row_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _row_view)| !self.frozen_rows.contains(&index))
                                    .map(|(index, row_view)| {
                                        row_view
                                            .view()
                                            .map(move |row_msg| Msg::RowMsg(index, row_msg))
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                        ],
                    ),
                ],
            )],
        )
    }
}
