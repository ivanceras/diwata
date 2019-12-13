use crate::widgets;
use sauron::{
    html::{attributes::*, events::*, units::*, *},
    Cmd, Component, Node,
};
use sqlparser::{dialect::GenericSqlDialect, sqlparser::Parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    ToggleShowQuery,
    ToggleShowRelatedTabs,
    ChangeQuickFind(String),
    QueryChanged(String),
    RunQuery,
}

pub struct ToolbarView {
    pub show_query: bool,
    pub show_related_tabs: bool,
    allocated_width: i32,
    allocated_height: i32,
    quick_find_search: String,
    pub sql_query: String,
    pub formatted_query: String,
}

impl ToolbarView {
    pub fn new() -> Self {
        ToolbarView {
            show_query: true,
            show_related_tabs: true,
            allocated_width: 0,
            allocated_height: 0,
            quick_find_search: String::new(),
            sql_query: String::new(),
            formatted_query: String::new(),
        }
    }

    pub fn set_sql_query(&mut self, sql_query: &Option<String>) {
        if let Some(sql_query) = sql_query {
            self.sql_query = sql_query.to_owned();
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
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ToggleShowQuery => self.show_query = !self.show_query,
            Msg::ToggleShowRelatedTabs => self.show_related_tabs = !self.show_related_tabs,
            Msg::ChangeQuickFind(search) => self.quick_find_search = search,
            Msg::QueryChanged(sql_query) => {
                trace!("Query is 1 changed to: {}", sql_query);
                self.sql_query = sql_query.to_string();
                let dialect = GenericSqlDialect {};
                trace!("Using generaic dialect");
                let statements = Parser::parse_sql(&dialect, sql_query);
                if let Ok(statements) = statements {
                    self.formatted_query = statements
                        .into_iter()
                        .map(|st| st.to_string())
                        .collect::<Vec<String>>()
                        .join(";\n");
                } else {
                    self.formatted_query = "ERROR".to_string();
                }
            }
            Msg::RunQuery => {
                trace!("Running sql_query: {}", self.sql_query);
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        section(
            vec![class("toolbar_and_query_view")],
            vec![
                header(
                    vec![class("toolbar")],
                    vec![
                        widgets::quick_find(35, oninput(|input| Msg::ChangeQuickFind(input.value))),
                        button(vec![], vec![text("Create new record")]),
                        button(vec![], vec![text("Insert new record")]),
                        button(vec![], vec![text("Save")]),
                        button(vec![], vec![text("Cancel")]),
                        button(vec![], vec![text("Delete")]),
                        button(vec![], vec![text("Refresh")]),
                        button(vec![], vec![text("Clear filter")]),
                        button(vec![], vec![text("Filter more..")]),
                        button(vec![], vec![text("Sort..")]),
                        button(vec![], vec![text("Export")]),
                        button(
                            vec![onclick(|_| Msg::ToggleShowQuery)],
                            vec![
                                input(vec![r#type("checkbox")], vec![]).add_attributes(attrs_flag(
                                    vec![("checked", "checked", self.show_query)],
                                )),
                                text("Show query"),
                            ],
                        ),
                        button(
                            vec![onclick(|_| Msg::ToggleShowRelatedTabs)],
                            vec![
                                input(vec![r#type("checkbox")], vec![]).add_attributes(attrs_flag(
                                    vec![("checked", "checked", self.show_related_tabs)],
                                )),
                                text("Show related records"),
                            ],
                        ),
                    ],
                ),
                section(
                    vec![
                        class("query_input"),
                        styles_flag(vec![("display", "none", !self.show_query)]),
                    ],
                    vec![
                        textarea(
                            vec![
                                class("sql_input"),
                                onchange(|input| Msg::QueryChanged(input.value)),
                                styles(vec![
                                    ("width", px(self.calculate_sql_input_width())),
                                    ("height", px(self.calculate_sql_input_height())),
                                ]),
                                value(&self.sql_query),
                                placeholder("SELECT * "),
                            ],
                            vec![],
                        ),
                        button(
                            vec![
                                class("run_query"),
                                onclick(|_| Msg::RunQuery),
                                styles(vec![
                                    ("width", px(self.run_query_button_width())),
                                    ("height", px(self.run_query_button_height())),
                                ]),
                            ],
                            vec![text("Run query")],
                        ),
                        textarea(
                            vec![
                                class("parsed_sql"),
                                readonly(true),
                                styles(vec![
                                    ("width", px(self.calculate_parsed_sql_width())),
                                    ("height", px(self.calculate_parsed_sql_height())),
                                ]),
                            ],
                            vec![text(&self.formatted_query)],
                        ),
                    ],
                ),
            ],
        )
    }
}
