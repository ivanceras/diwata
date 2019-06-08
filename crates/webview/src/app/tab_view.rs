use crate::app::{
    self,
    detail_view::{self, DetailView},
    table_view::{self, TableView},
};
use diwata_intel::{
    data_container::{FrozenData, Page},
    DataRow,
};

use diwata_intel::{Tab, TableName};
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    TableMsg(table_view::Msg),
    DetailViewMsg(detail_view::Msg),
}

pub struct TabView {
    pub name: String,
    pub table_name: TableName,
    detail_view: DetailView,
    pub table_view: TableView,
    pub is_visible: bool,
    /// is this a main tab
    /// is a one_one tab
    /// one_one_tab should only contain at most 1 datarow and is on detail view
    is_one_one: bool,
}

impl TabView {
    pub fn new(tab: Tab) -> Self {
        TabView {
            name: tab.name.clone(),
            table_name: tab.table_name.clone(),
            table_view: TableView::from_tab(tab),
            detail_view: DetailView::new(),
            is_visible: true,
            is_one_one: false,
        }
    }

    pub fn set_pages(&mut self, pages: &Vec<Page>, current_page: usize, total_records: usize) {
        let all_rows:Vec<DataRow> = pages.iter().fold(vec![], |mut acc, page|{
            acc.extend(page.rows.clone());
            acc
            });
        self.set_data_rows(&all_rows, current_page, total_records);
    }
    pub fn need_next_page(&self) -> bool {
        self.table_view.need_next_page()
    }

    /// this is a one one tab and should have only 1 record
    pub fn set_one_one_record(&mut self, data_row: &Option<DataRow>, total_rows: usize) {
        //assert!(self.is_one_one);
        if let Some(data_row) = data_row {
            // there should only be 1 record here
            self.set_data_rows(&vec![data_row.to_owned()], 1, total_rows);
        }
    }

    pub fn set_data_rows(
        &mut self,
        data_row: &Vec<DataRow>,
        current_page: usize,
        total_rows: usize,
    ) {
        self.table_view
            .set_data_rows(&data_row, current_page, total_rows);
        self.update_view();
    }

    pub fn freeze_rows(&mut self, rows: &Vec<usize>) {
        self.table_view.freeze_rows(rows);
    }

    pub fn freeze_columns(&mut self, columns: &Vec<usize>) {
        self.table_view.freeze_columns(columns);
    }
    pub fn set_frozen_data(&mut self, frozen_data: &FrozenData) {
        self.freeze_rows(&frozen_data.frozen_rows);
        self.freeze_columns(&frozen_data.frozen_columns);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show_detail_view(&mut self, row_index: usize) {
        self.detail_view.show();
        let fields = &self.table_view.row_views[row_index].fields;
        self.detail_view.set_fields(fields);
        self.detail_view.set_row(row_index);
    }
    /// Important NOTE: Don't remove views,
    /// just hide them, otherwise the DOM closures
    /// will be lost causing panics in the browser
    pub fn close_detail_view(&mut self) {
        self.detail_view.hide();
    }
    pub fn in_detail_view(&self) -> bool {
        self.detail_view.is_visible
    }

    /// calculate top of the clicked row, this will be used
    /// as the basis of where the detail_view row animation starts
    #[allow(unused)]
    fn clicked_row_top(&self) -> i32 {
        if let Some(row_index) = self.detail_view.row_index {
            let row_height = 30;
            (row_index as i32 * row_height) - self.table_view.scroll_top
        } else {
            0
        }
    }

    pub fn set_table_size(&mut self, size: (i32, i32)) {
        self.table_view.set_allocated_size(size);
    }

    pub fn set_is_one_one(&mut self, is_one_one: bool) {
        self.is_one_one = is_one_one;
        self.update_view();
    }

    /// check whether to be displayed in detail view if this
    /// a one one tab with only 1 record
    fn update_view(&mut self) {
        if self.is_one_one {
            if self.table_view.row_views.len() == 1 {
                sauron::log!("Succeed one_one_tab");
                self.show_detail_view(0);
            } else {
                sauron::log!(
                    "There should be 1 data row in one_one_tab, got{} ",
                    self.table_view.row_views.len()
                );
            }
        }
    }
    pub fn update(&mut self, msg: Msg) -> app::Cmd {
        match msg {
            Msg::TableMsg(table_msg) => self.table_view.update(table_msg),
            Msg::DetailViewMsg(detail_msg) => {
                self.detail_view.update(detail_msg);
                app::Cmd::none()
            }
        }
    }
    pub fn view(&self) -> Node<Msg> {
        section(
            [
                class("tab_view"),
                // to ensure no reusing of tab view when replaced with
                // another tab
                key(format!("tab_{}", self.name)),
                classes_flag([("in_detail_view", self.in_detail_view())]),
                styles_flag([("display", "none", !self.is_visible)]),
            ],
            [
                section(
                    [
                        class("detail_view_container"),
                        //classes_flag([("animate_detail_view", self.detail_view.is_visible)]),
                        // This is set here and extracted in attr(margin_top px) in css
                        // expand_detail_view animation
                        //attr("margin_top", self.clicked_row_top()),
                        //styles([("margin-top", px(self.clicked_row_top()))]),
                    ],
                    [self.detail_view.view().map(Msg::DetailViewMsg)],
                ),
                section(
                    [
                        class("table_view"),
                        styles_flag([("display", "none", self.detail_view.is_visible)]),
                    ],
                    [self.table_view.view().map(Msg::TableMsg)],
                ),
            ],
        )
    }
}
