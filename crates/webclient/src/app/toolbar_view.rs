use crate::widgets;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {
    ToggleShowQuery,
    ToggleShowRelatedTabs,
    ChangeQuickFind(String),
}

pub struct ToolbarView {
    pub show_query: bool,
    pub show_related_tabs: bool,
    allocated_width: i32,
    allocated_height: i32,
    quick_find_search: String,
}

impl ToolbarView {
    pub fn new() -> Self {
        ToolbarView {
            show_query: true,
            show_related_tabs: true,
            allocated_width: 0,
            allocated_height: 0,
            quick_find_search: String::new(),
        }
    }

    /// sql input size is resizable
    fn calculate_sql_input_size(&self) -> (i32, i32) {
        (self.allocated_width / 2 - 200, 90)
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

    pub fn set_allocated_size(&mut self, (width, height): (i32, i32)) {
        self.allocated_width = width;
        self.allocated_height = height;
    }

    /// the remaining width of the window width
    fn calculate_parsed_sql_size(&self) -> (i32, i32) {
        let (sql_input_width, _) = self.calculate_sql_input_size();
        let (run_query_width, _) = self.run_query_button_size();
        let parse_sql_width = self.allocated_width
            - (sql_input_width + run_query_width + self.calculate_needed_auxilliary_width());
        (parse_sql_width, 90)
    }
    fn calculate_parsed_sql_width(&self) -> i32 {
        self.calculate_parsed_sql_size().0
    }

    pub fn calculate_parsed_sql_height(&self) -> i32 {
        self.calculate_parsed_sql_size().1
    }

    fn calculate_needed_auxilliary_width(&self) -> i32 {
        50
    }

    pub fn get_consumed_height(&self) -> i32 {
        let mut consumed_heights = 0;
        consumed_heights += self.toolbar_icon_height();
        consumed_heights += if self.show_query {
            self.calculate_parsed_sql_height()
        } else {
            0
        };
        consumed_heights
    }

    fn toolbar_icon_height(&self) -> i32 {
        90
    }
}

impl Component<Msg> for ToolbarView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ToggleShowQuery => self.show_query = !self.show_query,
            Msg::ToggleShowRelatedTabs => self.show_related_tabs = !self.show_related_tabs,
            Msg::ChangeQuickFind(search) => self.quick_find_search = search,
        }
    }

    fn view(&self) -> Node<Msg> {
        section([class("toolbar_and_query_view")], [
                header(
                    [class("toolbar")],
                    [
                        widgets::quick_find(26, oninput(|input|Msg::ChangeQuickFind(input.value))),
                        button([], [text("Create new record")]),
                        button([], [text("Insert new record")]),
                        button([], [text("Save")]),
                        button([], [text("Cancel")]),
                        button([], [text("Delete")]),
                        button([], [text("Refresh")]),
                        button([], [text("Clear filter")]),
                        button([], [text("Filter more..")]),
                        button([], [text("Sort..")]),
                        button([], [text("Export")]),
                        button(
                            [onclick(|_| Msg::ToggleShowQuery)],
                            [
                                input([r#type("checkbox")], []).attributes(attrs_flag([(
                                    "checked",
                                    "checked",
                                    self.show_query,
                                )])),
                                text("Show query"),
                            ],
                        ),
                        button(
                            [onclick(|_| Msg::ToggleShowRelatedTabs)],
                            [
                                input([r#type("checkbox")], []).attributes(attrs_flag([(
                                    "checked",
                                    "checked",
                                    self.show_related_tabs,
                                )])),
                                text("Show related records"),
                            ],
                        ),
                    ],
                ),
                section(
                    [
                        class("query_input"),
                        styles_flag([("display", "none", !self.show_query)]),
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
                            [text("SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio.\n\
                                Eius eligendi aliquid beatae cumque ad illum.\n\
                                Deleniti suscipit non in consequatur.\n\
                                Doloremque beatae eum nulla praesentium cumque \n\
                                voluptatem quae tenetur.\n\
                                SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio.\n \
                                Eius eligendi aliquid beatae cumque ad illum.\n \
                                Deleniti suscipit non in consequatur. \n\
                                Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur\n\
                                SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. \n\
                                Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. \n\
                                Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.\n\
                                SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. \n\
                                Eius eligendi aliquid beatae cumque ad illum. Deleniti suscipit non in consequatur. \n\
                                Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.\n\
                                SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. \n\
                                Eius eligendi aliquid beatae cumque ad illum. \n\
                                Deleniti suscipit non in consequatur. \n\
                                Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.\n\
                                SELECT * FROM table\n\
                                Rem consequatur consectetur labore occaecati ipsa aut vel optio. \n\
                                Eius eligendi aliquid beatae cumque ad illum. \n\
                                Deleniti suscipit non in consequatur. \n\
                                Doloremque beatae eum nulla praesentium cumque voluptatem quae tenetur.\n\
                                  ")],
                        ),
                    ],
                ),
        ])
    }
}
