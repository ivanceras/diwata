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
    pub scroll_top: i32,
    scroll_left: i32,
    allocated_height: i32,
}

impl TableView {
    pub fn from_tab(tab: Tab, allocated_height: i32) -> Self {
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
            allocated_height,
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

    pub fn frozen_row_count(&self) -> usize {
        self.frozen_rows.len()
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

    /// This is the allocated height set by the parent tab
    pub fn set_allocated_height(&mut self, height: i32) {
        self.allocated_height = height;
    }

    /// TODO: include the height of the frozen rows
    pub fn calculate_normal_rows_height(&self) -> i32 {
        let height = self.allocated_height - Self::calculate_needed_height_for_auxilliary_spaces();
        if height < 0 {
            0
        } else {
            height
        }
    }

    /// height from the columns names, tab_links
    pub fn calculate_needed_height_for_auxilliary_spaces() -> i32 {
        50
    }

    /// These are values in a row that is under the frozen columns
    /// Can move up and down
    fn view_frozen_columns(&self) -> Node<Msg> {
        // can move up and down
        ol(
            [
                class("frozen_columns"),
                styles([("margin-top", px(-self.scroll_top))]),
            ],
            self.row_views
                .iter()
                .enumerate()
                .filter(|(index, _row_view)| !self.frozen_rows.contains(index))
                .map(|(index, row_view)| {
                    // The checkbox selection and the rows of the frozen
                    // columns
                    div(
                        [class("selector_and_frozen_column_row")],
                        [
                            input([r#type("checkbox")], []),
                            row_view
                                .view_frozen()
                                .map(move |row_msg| Msg::RowMsg(index, row_msg)),
                        ],
                    )
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// These are the columns of the frozen columns.
    /// Since frozen, they can not move in any direction
    fn view_frozen_column_names(&self) -> Node<Msg> {
        // absolutely immovable frozen column and row
        // can not move in any direction
        header(
            [class("frozen_column_names")],
            self.column_views
                .iter()
                .enumerate()
                .filter(|(index, _column)| self.frozen_columns.contains(index))
                .map(|(index, column)| {
                    column
                        .view()
                        .map(move |column_msg| Msg::ColumnMsg(index, column_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// The column names of the normal columns
    /// can move left and right and always follows the alignment of the column of the normal rows
    fn view_normal_column_names(&self) -> Node<Msg> {
        header(
            [class("normal_column_names")],
            self.column_views
                .iter()
                .enumerate()
                .filter(|(index, _column)| !self.frozen_columns.contains(index))
                .map(|(index, column)| {
                    column
                        .view()
                        .map(move |column_msg| Msg::ColumnMsg(index, column_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// The rows are both frozen row and frozen column
    /// Therefore can not move in any direction
    fn view_immovable_frozen_columns(&self) -> Node<Msg> {
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
                            row_view
                                .view_frozen()
                                .map(move |row_msg| Msg::RowMsg(index, row_msg)),
                        ],
                    )
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// These are the pinned columns
    fn view_frozen_rows(&self) -> Node<Msg> {
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
        )
    }

    /// The rest of the columns and move in any direction
    fn view_normal_rows(&self) -> Node<Msg> {
        // can move: left, right, up, down
        ol(
            [
                class("normal_rows"),
                styles([("height", px(self.calculate_normal_rows_height()))]),
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
        )
    }
}

impl Component<Msg> for TableView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::RowMsg(row_index, row_msg) => self.row_views[row_index].update(row_msg),
            Msg::ColumnMsg(column_index, column_msg) => {
                self.column_views[column_index].update(column_msg)
            }
            Msg::Scrolled((scroll_top, scroll_left)) => {
                self.scroll_top = scroll_top;
                self.scroll_left = scroll_left;
            }
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
                                    self.view_frozen_column_names(),
                                ],
                            ),
                            self.view_immovable_frozen_columns(),
                            // needed to overflow hide the frozen columns when scrolled up and down
                            section(
                                [
                                    class("frozen_columns_container"),
                                    styles([("height", px(self.calculate_normal_rows_height()))]),
                                ],
                                [self.view_frozen_columns()],
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
                                        self.view_normal_column_names(),
                                        self.view_frozen_rows(),
                                    ],
                                )],
                            ),
                            self.view_normal_rows(),
                        ],
                    ),
                ],
            )],
        )
    }
}
