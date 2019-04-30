use crate::app::{column_view::ColumnView, row_view::RowView};
use data_table::{DataColumn, DataTable};
use diwata_intel::{Field, Tab};
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::{column_view, row_view};
use data_table::DataRow;

#[derive(Clone)]
pub enum Msg {
    ColumnMsg(usize, column_view::Msg),
    RowMsg(usize, row_view::Msg),
    Scrolled((i32, i32)),
}

pub struct TableView {
    pub column_views: Vec<ColumnView>,
    pub row_views: Vec<RowView>,
    /// Which columns of the rows are to be frozen on the left side of the table
    frozen_rows: Vec<usize>,
    frozen_columns: Vec<usize>,
    scroll_top: i32,
    scroll_left: i32,
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
            scroll_top: 0,
            scroll_left: 0,
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
            scroll_top: 0,
            scroll_left: 0,
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
    /// TODO: also update the freeze_columns for each row_views
    pub fn set_data_rows(&mut self, data_row: Vec<DataRow>) {
        self.row_views = data_row.into_iter().map(RowView::new).collect();
        self.update_freeze_columns();
    }

    pub fn freeze_rows(&mut self, rows: Vec<usize>) {
        self.frozen_rows = rows;
    }
    /// Keep updating which columns are frozen
    /// call these when new rows are set or added
    pub fn update_freeze_columns(&mut self) {
        let frozen_columns = self.frozen_columns.clone();
        self.row_views
            .iter_mut()
            .for_each(|row_view| row_view.freeze_columns(frozen_columns.clone()))
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        self.frozen_columns = columns.clone();
        self.update_freeze_columns();
    }
}

impl Component<Msg> for TableView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Scrolled((scroll_top, scroll_left)) => {
                sauron::log!("table is scrolled ({},{})", scroll_top, scroll_left);
                self.scroll_top = scroll_top;
                self.scroll_left = scroll_left;
            }
            _ => {}
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [class("table")],
            [section(
                [class("rows_and_frozen_columns")],
                [
                    section(
                        [class("frozen_column_names_and_frozen_column_rows")],
                        [
                            section(
                                [class("spacer_and_frozen_column_names")],
                                [
                                    div([class("spacer")], [input([r#type("checkbox")], [])]),
                                    // absolutely immovable frozen column and row
                                    // can not move in any direction
                                    header(
                                        [class("frozen_column_names")],
                                        self.column_views
                                            .iter()
                                            .enumerate()
                                            .filter(|(index, _column)| {
                                                self.frozen_columns.contains(index)
                                            })
                                            .map(|(index, column)| {
                                                column.view().map(move |column_msg| {
                                                    Msg::ColumnMsg(index, column_msg)
                                                })
                                            })
                                            .collect::<Vec<Node<Msg>>>(),
                                    ),
                                ],
                            ),
                            ol(
                                [class("immovable_frozen_columns")],
                                self.row_views
                                    .iter()
                                    .enumerate()
                                    .filter(|(index, _row_view)| self.frozen_rows.contains(index))
                                    .map(|(index, row_view)| {
                                        div(
                                            [class("selector_and_frozen_column_row")],
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
                            // needed to overflow hide the frozen columns when scrolled up and down
                            section(
                                [class("frozen_columns_container")],
                                [
                                    // can move up and down
                                    ol(
                                        [
                                            class("frozen_columns"),
                                            styles([("margin-top", px(-self.scroll_top))]),
                                        ],
                                        self.row_views
                                            .iter()
                                            .enumerate()
                                            .filter(|(index, _row_view)| {
                                                !self.frozen_rows.contains(index)
                                            })
                                            .map(|(index, row_view)| {
                                                // The checkbox selection and the rows of the frozen
                                                // columns
                                                div(
                                                    [class("selector_and_frozen_column_row")],
                                                    [
                                                        input([r#type("checkbox")], []),
                                                        row_view.view_frozen().map(
                                                            move |row_msg| {
                                                                Msg::RowMsg(index, row_msg)
                                                            },
                                                        ),
                                                    ],
                                                )
                                            })
                                            .collect::<Vec<Node<Msg>>>(),
                                    ),
                                ],
                            ),
                        ],
                    ),
                    section(
                        [class("frozen_rows_and_normal_rows")],
                        [
                            section(
                                [class("normal_column_names_and_frozen_rows_container")],
                                [section(
                                    [
                                        class("normal_column_names_and_frozen_rows"),
                                        styles([("margin-left", px(-self.scroll_left))]),
                                    ],
                                    [
                                        // can move left and right
                                        header(
                                            [class("normal_column_names")],
                                            self.column_views
                                                .iter()
                                                .enumerate()
                                                .filter(|(index, _column)| {
                                                    !self.frozen_columns.contains(index)
                                                })
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
                                                .filter(|(index, _row_view)| {
                                                    self.frozen_rows.contains(&index)
                                                })
                                                .map(|(index, row_view)| {
                                                    row_view.view().map(move |row_msg| {
                                                        Msg::RowMsg(index, row_msg)
                                                    })
                                                })
                                                .collect::<Vec<Node<Msg>>>(),
                                        ),
                                    ],
                                )],
                            ),
                            // can move: left, right, up, down
                            ol(
                                [
                                    class("normal_rows"),
                                    onscroll(|scroll| Msg::Scrolled(scroll)),
                                ],
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
