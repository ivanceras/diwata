use crate::{
    app::{
        detail_view::{self, DetailView},
        field_view,
        row_view::{self, RowView},
        table_view::{self, TableView},
    },
    data::{FrozenData, Page},
};
use data_table::DataRow;

use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

use diwata_intel::Tab;

#[derive(Clone)]
pub enum Msg {
    TableMsg(table_view::Msg),
    DetailViewMsg(detail_view::Msg),
}

pub struct TabView {
    pub name: String,
    detail_view: DetailView,
    table_view: TableView,
    is_visible: bool,
}

impl TabView {
    pub fn new(tab: Tab) -> Self {
        TabView {
            name: tab.name.clone(),
            table_view: TableView::from_tab(tab),
            detail_view: DetailView::new(),
            is_visible: true,
        }
    }

    pub fn set_pages(&mut self, pages: Vec<Page>) {
        for page in pages {
            self.set_data_rows(page.rows);
        }
    }

    pub fn set_data_rows(&mut self, data_row: Vec<DataRow>) {
        self.table_view.set_data_rows(data_row);
    }

    pub fn freeze_rows(&mut self, rows: Vec<usize>) {
        self.table_view.freeze_rows(rows);
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        sauron::log!("freeze columns: {:?}", columns);
        self.table_view.freeze_columns(columns);
    }
    pub fn set_frozen_data(&mut self, frozen_data: FrozenData) {
        self.freeze_rows(frozen_data.frozen_rows);
        self.freeze_columns(frozen_data.frozen_columns);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    fn show_detail_view(&mut self, row_index: usize) {
        self.detail_view.show();
        let fields = &self.table_view.row_views[row_index].fields;
        self.detail_view.set_fields(fields);
        self.detail_view.set_row(row_index);
    }
    /// Important NOTE: Don't remove views,
    /// just hide them, otherwise the DOM closures
    /// will be lost causing panics in the browser
    fn close_detail_view(&mut self) {
        self.detail_view.hide();
    }
    pub fn in_detail_view(&self) -> bool {
        self.detail_view.is_visible
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TableMsg(table_view::Msg::RowMsg(row_index, row_view::Msg::DoubleClick)) => {
                self.show_detail_view(row_index);
            }
            Msg::TableMsg(table_msg) => self.table_view.update(table_msg),
            Msg::DetailViewMsg(detail_view::Msg::Close) => {
                self.close_detail_view();
            }
            Msg::DetailViewMsg(detail_msg) => {
                self.detail_view.update(detail_msg);
            }
        }
    }
    fn view(&self) -> Node<Msg> {
        section(
            [
                class("tab"),
                styles_flag([
                    ("display", "flex", self.is_visible),
                    ("display", "none", !self.is_visible),
                ]),
            ],
            [
                div([], [button([], [text(&self.name)])]),
                section(
                    [class("detail_view")],
                    [self.detail_view.view().map(Msg::DetailViewMsg)],
                ),
                section(
                    [class("table_view")],
                    [self.table_view.view().map(Msg::TableMsg)],
                ),
            ],
        )
    }
}
