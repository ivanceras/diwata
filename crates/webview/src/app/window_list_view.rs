use crate::{assets, widgets};
use diwata_intel::{window::GroupedWindow, TableName};
use sauron::{
    html::{attributes::*, events::*, units::*, *},
    Cmd, Component, Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    ChangeSearch(String),
    ReceiveWindowList(Vec<GroupedWindow>),
    ClickedWindow(TableName),
}

pub struct WindowListView {
    window_list: Vec<GroupedWindow>,
    allocated_width: i32,
    allocated_height: i32,
}

impl WindowListView {
    pub fn new(window_list: Vec<GroupedWindow>) -> Self {
        WindowListView {
            window_list,
            allocated_width: 0,
            allocated_height: 0,
        }
    }

    fn calculate_window_list_height(&self) -> i32 {
        self.allocated_height - 20
    }

    fn calculate_window_list_width(&self) -> i32 {
        self.allocated_width
    }

    pub fn set_allocated_size(&mut self, (width, height): (i32, i32)) {
        self.allocated_width = width;
        self.allocated_height = height;
    }
}

impl Component<Msg> for WindowListView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeSearch(search) => {
                trace!("searching for {}", search);
            }
            Msg::ReceiveWindowList(window_list) => {
                self.window_list = window_list;
            }
            Msg::ClickedWindow(table_name) => {
                trace!("Opening window: {}", table_name.complete_name());
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                section(
                    vec![class("window_list_search")],
                    vec![widgets::search_widget(oninput(|input| {
                        Msg::ChangeSearch(input.value)
                    }))],
                ),
                section(
                    vec![
                        class("window_list"),
                        styles(vec![
                            ("height", px(self.calculate_window_list_height())),
                            ("width", px(self.calculate_window_list_width())),
                        ]),
                    ],
                    self.window_list
                        .iter()
                        .map(|group| {
                            ul(
                                vec![],
                                vec![
                                    li(
                                        vec![class("window_list_group_name")],
                                        vec![text(&group.group)],
                                    ),
                                    ul(
                                        vec![],
                                        group
                                            .window_names
                                            .iter()
                                            .map(|win_name| {
                                                let table_name: TableName =
                                                    win_name.table_name.clone();
                                                li(
                                                    vec![key(table_name.complete_name())],
                                                    vec![a(
                                                        vec![
                                                            href(format!(
                                                                "/{}",
                                                                &table_name.complete_name()
                                                            )),
                                                            class("window_list_link"),
                                                            onclick(move |_| {
                                                                Msg::ClickedWindow(
                                                                    table_name.clone(),
                                                                )
                                                            }),
                                                        ],
                                                        vec![
                                                            span(
                                                                vec![class("table_icon")],
                                                                vec![assets::svg_table_icon()],
                                                            ),
                                                            text(&win_name.name),
                                                        ],
                                                    )],
                                                )
                                            })
                                            .collect::<Vec<Node<Msg>>>(),
                                    ),
                                ],
                            )
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
