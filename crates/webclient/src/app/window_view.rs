use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::tab_view::{self, TabView};
use data_table::{DataRow, Value};
use diwata_intel::{IndirectTab, TableName, Window};

#[derive(Clone)]
pub enum Msg {
    MainTabMsg(tab_view::Msg),
    OneOneTabMsg(usize, tab_view::Msg),
    HasManyTabMsg(usize, tab_view::Msg),
    IndirectTabMsg(usize, (TableName, tab_view::Msg)),
    ShowHasManyTab(usize),
    ShowIndirectTab(usize),
}

pub struct WindowView {
    pub name: String,
    main_tab: TabView,
    has_one_tabs: Vec<TabView>,
    one_one_tabs: Vec<TabView>,
    has_many_tabs: Vec<TabView>,
    indirect_tabs: Vec<(TableName, TabView)>,
    is_visible: bool,
    active_has_many_tab: Option<usize>,
    active_indirect_tab: Option<usize>,
}

impl WindowView {
    pub fn new(window: Window) -> Self {
        let mut window_view = WindowView {
            name: window.name,
            main_tab: {
                let mut tab_view = TabView::new(window.main_tab);
                tab_view.set_data_rows(Self::make_sample_rows());
                tab_view
            },
            has_one_tabs: window.has_one_tabs.into_iter().map(TabView::new).collect(),
            one_one_tabs: window.one_one_tabs.into_iter().map(TabView::new).collect(),
            has_many_tabs: window.has_many_tabs.into_iter().map(TabView::new).collect(),
            indirect_tabs: window
                .indirect_tabs
                .into_iter()
                .map(|tab| (tab.linker, TabView::new(tab.tab)))
                .collect(),
            is_visible: true,
            active_has_many_tab: Some(0),
            active_indirect_tab: None,
        };
        window_view.update_active_has_many_or_indirect_tab();
        window_view
    }

    fn update_active_has_many_or_indirect_tab(&mut self) {
        let active_has_many_tab = self.active_has_many_tab;
        let active_indirect_tab = self.active_indirect_tab;

        self.has_many_tabs
            .iter_mut()
            .enumerate()
            .for_each(|(index, tab)| {
                if active_has_many_tab == Some(index) {
                    tab.show();
                } else {
                    tab.hide();
                }
            });
        self.indirect_tabs
            .iter_mut()
            .enumerate()
            .for_each(|(index, (table_name, tab))| {
                if active_indirect_tab == Some(index) {
                    tab.show();
                } else {
                    tab.hide();
                }
            });
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    fn make_sample_rows() -> Vec<DataRow> {
        (0..10)
            .into_iter()
            .map(|n| Self::make_sample_row(n))
            .collect()
    }
    fn make_sample_row(row: usize) -> Vec<Value> {
        (0..5)
            .into_iter()
            .map(|n| Value::Text(format!("Row{}-Value{}", row, n)))
            .collect()
    }

    fn activate_has_many_tab(&mut self, index: usize) {
        self.active_has_many_tab = Some(index);
        self.active_indirect_tab = None;
        self.update_active_has_many_or_indirect_tab();
    }
    fn activate_indirect_tab(&mut self, index: usize) {
        self.active_has_many_tab = None;
        self.active_indirect_tab = Some(index);
        self.update_active_has_many_or_indirect_tab();
    }
}

impl Component<Msg> for WindowView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::MainTabMsg(tab_msg) => self.main_tab.update(tab_msg),
            Msg::OneOneTabMsg(index, tab_msg) => self.one_one_tabs[index].update(tab_msg),
            Msg::HasManyTabMsg(index, tab_msg) => self.has_many_tabs[index].update(tab_msg),
            Msg::IndirectTabMsg(index, (table_name, tab_msg)) => {
                self.indirect_tabs[index].1.update(tab_msg)
            }
            Msg::ShowHasManyTab(index) => self.activate_has_many_tab(index),
            Msg::ShowIndirectTab(index) => self.activate_indirect_tab(index),
        }
    }
    fn view(&self) -> Node<Msg> {
        main(
            [
                class("window"),
                styles_flag([
                    ("display", "block", self.is_visible),
                    ("display", "none", !self.is_visible),
                ]),
            ],
            [
                header(
                    [class("query_input")],
                    [
                        h2([], [button([], [text(&self.name)])]),
                        textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
                    ],
                ),
                section(
                    [class("main_tab")],
                    [self.main_tab.view().map(Msg::MainTabMsg)],
                ),
                section(
                    [class("one_one_tabs")],
                    self.one_one_tabs
                        .iter()
                        .enumerate()
                        .map(|(index, tab)| {
                            TabView::view(tab).map(move |tab_msg| Msg::OneOneTabMsg(index, tab_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                header(
                    [class("has_many_and_indirect_tabs_link")],
                    [
                        nav(
                            [class("has_many_tabs_link")],
                            self.has_many_tabs
                                .iter()
                                .enumerate()
                                .map(|(index, tab)| {
                                    button(
                                        [onclick(move |_| Msg::ShowHasManyTab(index))],
                                        [text(&tab.name)],
                                    )
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                        nav(
                            [class("indirect_tabs_link")],
                            self.indirect_tabs
                                .iter()
                                .enumerate()
                                .map(|(index, (table_name, tab))| {
                                    button(
                                        [onclick(move |_| Msg::ShowIndirectTab(index))],
                                        [text(&tab.name)],
                                    )
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                    ],
                ),
                section(
                    [class("has_many_and_indirect_tabs")],
                    [
                        section(
                            [class("has_many_tabs")],
                            self.has_many_tabs
                                .iter()
                                .enumerate()
                                .map(|(index, tab)| {
                                    TabView::view(tab)
                                        .map(move |tab_msg| Msg::HasManyTabMsg(index, tab_msg))
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                        section(
                            [class("indirect_tabs")],
                            self.indirect_tabs
                                .iter()
                                .enumerate()
                                .map(|(index, (table_name, tab))| {
                                    let table_name = table_name.clone();
                                    TabView::view(tab).map(move |tab_msg| {
                                        Msg::IndirectTabMsg(index, (table_name.clone(), tab_msg))
                                    })
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                    ],
                ),
            ],
        )
    }
}
