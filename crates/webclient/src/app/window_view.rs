use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::tab_view::{self, TabView};
use diwata_intel::{IndirectTab, TableName, Window};

#[derive(Debug, Clone)]
pub enum Msg {
    MainTabMsg(tab_view::Msg),
    HasManyTabMsg(usize, tab_view::Msg),
    IndirectTabMsg(usize, TableName, tab_view::Msg),
}

pub struct WindowView {
    pub name: String,
    main_tab: TabView,
    has_one_tabs: Vec<TabView>,
    one_one_tabs: Vec<TabView>,
    has_many_tabs: Vec<TabView>,
    indirect_tabs: Vec<(TableName, TabView)>,
    is_visible: bool,
}

impl WindowView {
    pub fn new(window: Window) -> Self {
        WindowView {
            name: window.name,
            main_tab: TabView::new(window.main_tab),
            has_one_tabs: window.has_one_tabs.into_iter().map(TabView::new).collect(),
            one_one_tabs: window.one_one_tabs.into_iter().map(TabView::new).collect(),
            has_many_tabs: window.has_many_tabs.into_iter().map(TabView::new).collect(),
            indirect_tabs: window
                .indirect_tabs
                .into_iter()
                .map(|tab| (tab.linker, TabView::new(tab.tab)))
                .collect(),
            is_visible: true,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
}

impl Component<Msg> for WindowView {
    fn update(&mut self, msg: Msg) {}
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
                    [class("datawindow")],
                    [h2([], [button([], [text(&self.name)])])],
                ),
                textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
                section(
                    [class("main-tab")],
                    [self.main_tab.view().map(Msg::MainTabMsg)],
                ),
                section(
                    [class("has-many-tabs")],
                    self.has_many_tabs
                        .iter()
                        .enumerate()
                        .map(|(index, tab)| {
                            TabView::view(tab)
                                .map(move |tab_msg| Msg::HasManyTabMsg(index, tab_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                aside(
                    [class("indirect-tabs")],
                    self.indirect_tabs
                        .iter()
                        .enumerate()
                        .map(|(index, (table_name, tab))| {
                            let table_name = table_name.clone();
                            TabView::view(tab).map(move |tab_msg| {
                                Msg::IndirectTabMsg(index, table_name.clone(), tab_msg)
                            })
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
