use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::{
    app::{
        tab_view::{self, TabView},
        toolbar_view::{self, ToolbarView},
    },
    data::WindowData,
};
use diwata_intel::{TableName, Window};

pub struct WindowView {
    pub name: String,
    main_tab: TabView,
    one_one_tabs: Vec<TabView>,
    has_many_tabs: Vec<TabView>,
    indirect_tabs: Vec<(TableName, TabView)>,
    pub is_visible: bool,
    active_has_many_tab: Option<usize>,
    active_indirect_tab: Option<usize>,
    browser_height: i32,
    browser_width: i32,
    show_sql_input: bool,
    toolbar_view: ToolbarView,
}

#[derive(Clone)]
pub enum Msg {
    MainTabMsg(tab_view::Msg),
    OneOneTabMsg(usize, tab_view::Msg),
    HasManyTabMsg(usize, tab_view::Msg),
    IndirectTabMsg(usize, (TableName, tab_view::Msg)),
    ShowHasManyTab(usize),
    ShowIndirectTab(usize),
    BrowserResized(i32, i32),
    ToolbarMsg(toolbar_view::Msg),
    CloseDetailView,
}

impl Component<Msg> for WindowView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::MainTabMsg(tab_msg) => self.main_tab.update(tab_msg),
            Msg::OneOneTabMsg(index, tab_msg) => self.one_one_tabs[index].update(tab_msg),
            Msg::HasManyTabMsg(index, tab_msg) => self.has_many_tabs[index].update(tab_msg),
            Msg::IndirectTabMsg(index, (_table_name, tab_msg)) => {
                self.indirect_tabs[index].1.update(tab_msg)
            }
            Msg::ShowHasManyTab(index) => self.activate_has_many_tab(index),
            Msg::ShowIndirectTab(index) => self.activate_indirect_tab(index),
            Msg::BrowserResized(width, height) => {
                sauron::log!("resized: {},{}", width, height);
                self.browser_width = width;
                self.browser_height = height;
                self.update_size_allocation();
            }
            Msg::ToolbarMsg(toolbar_msg) => self.toolbar_view.update(toolbar_msg),
            Msg::CloseDetailView => self.close_detail_view(),
        }
    }
    fn view(&self) -> Node<Msg> {
        main(
            [
                class("window"),
                styles_flag([("display", "none", !self.is_visible)]),
            ],
            [
                header(
                    [class("toolbar_view")],
                    [self.toolbar_view.view().map(Msg::ToolbarMsg)],
                ),
                header(
                    [
                        class("query_input"),
                        styles_flag([("display", "none", !self.toolbar_view.show_query)]),
                    ],
                    [
                        textarea(
                            [
                                class("sql_input"),
                                styles([
                                    ("width", px(self.calculate_sql_input_width())),
                                    ("height", px(self.calculate_sql_input_height())),
                                ]),
                                placeholder("SELECT * "),
                            ],
                            [],
                        ),
                        button([class("run_query"),
                               styles([
                                      ("width", px(self.run_query_button_width())),
                                      ("height", px(self.run_query_button_height()))
                               ])
                        ], [text("Run query")]),
                        textarea(
                            [
                                class("parsed_sql"),
                                readonly(true),
                                styles([
                                    ("width", px(self.calculate_parsed_sql_width())),
                                    ("height", px(self.calculate_parsed_sql_height())),
                                ]),
                            ],
                            [text("SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.
                                SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.SELECT * FROM table
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.
                                  ")],
                        ),
                    ],
                ),
                section([class("main_tab_and_one_one_tabs_and_detail_close_btn")], [
                        section(
                            [
                                class("main_tab_and_one_one_tabs"),
                                styles([
                                    ("width", px(self.calculate_detail_window_width())),
                                    ("height", px(self.calculate_detail_window_height())),
                                ]),
                                // show only the scrollbar when in detailed view
                                // to prevent double scrolling when table_view is shown
                                styles_flag([("overflow", "auto", self.in_detail_view())]),
                            ],
                            [
                                section(
                                    [class("main_tab")],
                                    [self.main_tab.view().map(Msg::MainTabMsg)],
                                ),
                                section(
                                    [
                                        class("one_one_tabs"),
                                        styles_flag([
                                            ("display", "flex", self.in_detail_view()),
                                            ("display", "none", !self.in_detail_view()),
                                        ]),
                                    ],
                                    self.one_one_tabs
                                        .iter()
                                        .enumerate()
                                        .map(|(index, tab)| {
                                            details(
                                                [class("one_one_tab")],
                                                [
                                                    sauron::html::summary([], [text(&tab.name)]),
                                                    TabView::view(tab).map(move |tab_msg| {
                                                        Msg::OneOneTabMsg(index, tab_msg)
                                                    }),
                                                ],
                                            )
                                        })
                                        .collect::<Vec<Node<Msg>>>(),
                                ),
                            ],
                        ),
                        button([class("close_detail_btn"),
                            onclick(|_|Msg::CloseDetailView),
                        ], [text("X")]),
                ]),
                section(
                    [
                        class("detail_row_related_records"),
                        styles_flag([
                            ("display", "block", self.in_detail_view()),
                            ("display", "none", !self.in_detail_view()),
                        ]),
                    ],
                    [
                        header(
                            [class("has_many_and_indirect_tabs_link")],
                            [
                                nav(
                                    [class("has_many_tabs_link")],
                                    self.has_many_tabs
                                        .iter()
                                        .enumerate()
                                        .map(|(index, tab)| {
                                            a(
                                                [
                                                    class("tab_links"),
                                                    classes_flag([("active", tab.is_visible)]),
                                                    onclick(move |_| Msg::ShowHasManyTab(index)),
                                                ],
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
                                        .map(|(index, (_table_name, tab))| {
                                            a(
                                                [
                                                    class("tab_links"),
                                                    classes_flag([("active", tab.is_visible)]),
                                                    onclick(move |_| Msg::ShowIndirectTab(index)),
                                                ],
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
                                            TabView::view(tab).map(move |tab_msg| {
                                                Msg::HasManyTabMsg(index, tab_msg)
                                            })
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
                                                Msg::IndirectTabMsg(
                                                    index,
                                                    (table_name.clone(), tab_msg),
                                                )
                                            })
                                        })
                                        .collect::<Vec<Node<Msg>>>(),
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        )
    }
}

impl WindowView {
    pub fn new(window: Window, browser_width: i32, browser_height: i32) -> Self {
        let mut window_view = WindowView {
            name: window.name,
            main_tab: TabView::new(window.main_tab),
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
            browser_width,
            browser_height,
            show_sql_input: false,
            toolbar_view: ToolbarView::new(),
        };
        window_view.update_active_has_many_or_indirect_tab();
        window_view.update_size_allocation();
        window_view
    }

    /// Important: set the data rows first before setting the frozen data
    pub fn set_window_data(&mut self, window_data: WindowData) {
        let WindowData {
            main_tab_data,
            main_tab_frozen_data,
            one_one_tab_data,

            has_many_tab_data,
            has_many_tab_frozen_data,

            indirect_tab_data,
            indirect_tab_frozen_data,
        } = window_data;
        self.main_tab.set_pages(main_tab_data);
        self.main_tab.set_frozen_data(main_tab_frozen_data);

        // one one tab should only have 1 row
        for (index, row) in one_one_tab_data.into_iter().enumerate() {
            self.one_one_tabs[index].set_data_rows(vec![row]);
        }

        for (index, pages) in has_many_tab_data.into_iter().enumerate() {
            self.has_many_tabs[index].set_pages(pages);
            self.has_many_tabs[index].set_frozen_data(has_many_tab_frozen_data[index].clone());
        }

        for (index, pages) in indirect_tab_data.into_iter().enumerate() {
            self.indirect_tabs[index].1.set_pages(pages);
            self.indirect_tabs[index]
                .1
                .set_frozen_data(indirect_tab_frozen_data[index].clone());
        }
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
            .for_each(|(index, (_table_name, tab))| {
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

    /// Window height is the browser height - the spacers
    fn calculate_window_size(&self) -> (i32, i32) {
        let width = self.browser_width - self.calculate_needed_width_for_auxilliary_spaces();
        let height = self.browser_height - self.calculate_needed_height_for_auxilliary_spaces();
        (width, height)
    }

    /// split the browser height if in detail view
    /// use the broser height when in no detail view is there
    /// clamped to 0 if the height becomes negative
    fn calculate_main_table_size(&self) -> (i32, i32) {
        let (window_width, window_height) = self.calculate_window_size();
        let (_related_tab_width, related_tab_height) = self.calculate_related_tabs_size();

        let main_table_height = if self.in_detail_view() {
            window_height - related_tab_height
        } else {
            window_height
        };

        let clamped_main_table_height = if main_table_height < 0 {
            0
        } else {
            main_table_height
        };
        (window_width, clamped_main_table_height)
    }

    /// the detail view takes up the main table height
    fn calculate_detail_window_size(&self) -> (i32, i32) {
        self.calculate_main_table_size()
    }

    fn calculate_detail_window_height(&self) -> i32 {
        self.calculate_detail_window_size().1
    }

    /// sql input size is resizable
    fn calculate_sql_input_size(&self) -> (i32, i32) {
        let (window_width, _) = self.calculate_window_size();
        (window_width/2, 90)
    }

    fn calculate_sql_input_width(&self) -> i32 {
        self.calculate_sql_input_size().0
    }

    fn calculate_sql_input_height(&self) -> i32 {
        self.calculate_sql_input_size().1
    }

    /// fix run button size
    fn run_query_button_size(&self) -> (i32, i32) {
        (100, 40)
    }

    fn run_query_button_width(&self) -> i32 {
        self.run_query_button_size().0
    }

    fn run_query_button_height(&self) -> i32 {
        self.run_query_button_size().1
    }

    /// the remaining width of the window width
    fn calculate_parsed_sql_size(&self) -> (i32, i32) {
        let (window_width, _) = self.calculate_window_size();
        let (sql_input_width, _) = self.calculate_sql_input_size();
        let (run_query_width, _) = self.run_query_button_size();
        let parse_sql_width = window_width - (sql_input_width + run_query_width);
        (parse_sql_width, 90)
    }
    fn calculate_parsed_sql_width(&self) -> i32 {
        self.calculate_parsed_sql_size().0
    }
    fn calculate_parsed_sql_height(&self) -> i32 {
        self.calculate_parsed_sql_size().1
    }

    fn calculate_detail_window_width(&self) -> i32 {
        self.calculate_detail_window_size().0
    }

    fn calculate_related_tabs_size(&self) -> (i32, i32) {
        (
            self.calculate_related_tabs_width(),
            self.calculate_related_tabs_height(),
        )
    }

    /// fix the related tab heights and the user can also adjust this
    /// up and down
    fn calculate_related_tabs_height(&self) -> i32 {
        300
    }

    fn calculate_related_tabs_width(&self) -> i32 {
        self.browser_width - self.calculate_needed_width_for_auxilliary_spaces()
    }

    /// height needed for the toolbars, columns, sql textarea, paddings and margins
    fn calculate_needed_height_for_auxilliary_spaces(&self) -> i32 {
        190
    }

    /// this includes the window_list width, and left padding and margins
    fn calculate_needed_width_for_auxilliary_spaces(&self) -> i32 {
        300
    }

    /// TODO: also call this when detail view is closed to recalculate the sizes
    fn update_size_allocation(&mut self) {
        let calculated_main_table_size = self.calculate_main_table_size();
        let calculated_related_tabs_size = self.calculate_related_tabs_size();

        self.main_tab.set_table_size(calculated_main_table_size);
        self.one_one_tabs
            .iter_mut()
            .for_each(|tab| tab.set_table_size(calculated_main_table_size));
        self.has_many_tabs
            .iter_mut()
            .for_each(|tab| tab.set_table_size(calculated_related_tabs_size));
        self.indirect_tabs
            .iter_mut()
            .for_each(|(_table_name, tab)| tab.set_table_size(calculated_related_tabs_size));
    }

    fn in_detail_view(&self) -> bool {
        self.main_tab.in_detail_view()
    }

    fn close_detail_view(&mut self) {
        self.main_tab.close_detail_view()
    }
}
