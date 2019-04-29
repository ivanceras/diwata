use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::{tab_view, TabView};
use diwata_intel::{TableName, Window};

#[derive(Debug, Clone)]
pub enum Msg {
    TabMsg(tab_view::Msg),
}

pub struct WindowView {
    main_tab: TabView,
    has_one_tabs: Vec<TabView>,
    one_one_tabs: Vec<TabView>,
    has_many_tabs: Vec<TabView>,
    indirect_tabs: Vec<(TableName, TabView)>,
}

impl WindowView {
    pub fn new(window: Window) -> Self {
        WindowView {
            main_tab: TabView::new(window.main_tab),
            has_one_tabs: window.has_one_tabs.into_iter().map(TabView::new).collect(),
            one_one_tabs: window.one_one_tabs.into_iter().map(TabView::new).collect(),
            has_many_tabs: window.has_many_tabs.into_iter().map(TabView::new).collect(),
            indirect_tabs: window
                .indirect_tabs
                .into_iter()
                .map(|(table_name, tab)| (table_name, TabView::new(tab)))
                .collect(),
        }
    }
}

impl Component<Msg> for WindowView {
    fn update(&mut self, msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        div([class("datawindow")], [])
    }
}
