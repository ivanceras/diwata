use crate::data::WindowData;
use diwata_intel::{
    window::{GroupedWindow},
    Window,
};
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};
use window_view::WindowView;

mod column_view;
mod detail_view;
mod field_view;
mod row_view;
mod tab_view;
mod table_view;
mod window_view;

#[derive(Clone)]
pub enum Msg {
    ActivateWindow(usize),
    WindowMsg(usize, window_view::Msg),
    BrowserResized(i32, i32),
    Tick,
}

pub struct App {
    window_views: Vec<WindowView>,
    window_list: Vec<GroupedWindow>,
    active_window: usize,
    browser_height: i32,
    browser_width: i32,
}

impl App {
    pub fn new(
        window_list: Vec<GroupedWindow>,
        windows: Vec<Window>,
        browser_width: i32,
        browser_height: i32,
    ) -> App {
        let mut app = App {
            window_views: windows
                .into_iter()
                .map(|window| WindowView::new(window, browser_width, browser_height))
                .collect(),
            window_list,
            active_window: 0,
            browser_width,
            browser_height,
        };
        app.update_active_window();
        app
    }

    pub fn set_window_data(&mut self, index: usize, window_data: WindowData) {
        self.window_views[index].set_window_data(window_data);
    }

    fn update_active_window(&mut self) {
        let active_window = self.active_window;
        self.window_views
            .iter_mut()
            .enumerate()
            .for_each(|(index, window)| {
                if index == active_window {
                    window.show()
                } else {
                    window.hide()
                }
            })
    }

    fn activate_window(&mut self, index: usize) {
        self.active_window = index;
        self.update_active_window();
    }

    fn calculate_window_list_height(&self) -> i32 {
        self.browser_height - self.calculate_needed_auxilliary_spaces()
    }

    fn calculate_needed_auxilliary_spaces(&self) -> i32 {
        50
    }

    fn calculate_window_list_width(&self) -> i32 {
        200
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ActivateWindow(index) => self.activate_window(index),
            Msg::WindowMsg(index, window_msg) => self.window_views[index].update(window_msg),
            Msg::BrowserResized(width, height) => {
                sauron::log!("Browser is resized to: {}, {}", width, height);
                self.browser_width = width;
                self.browser_height = height;
                //also notify all opened windows with the resize;
                self.window_views.iter_mut().for_each(|window| {
                    window.update(window_view::Msg::BrowserResized(width, height))
                });
            }
            Msg::Tick => {
                sauron::log("Ticking");
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [class("app")],
            [
                // TOP-LEFT: Content 1
                header([class("logo")], [h1([], [text("Diwata")])]),
                // TOP-RIGHT: Content 2
                header([class("user_credentials")], [text("User")]),
                // BOTTOM-LEFT: Content 3
                section(
                    [
                        class("window_list"),
                        styles([
                            ("height", px(self.calculate_window_list_height())),
                            ("width", px(self.calculate_window_list_width())),
                        ]),
                    ],
                    self.window_list
                        .iter()
                        .map(|group| {
                            ul(
                                [],
                                [
                                    li([], [text(&group.group)]),
                                    ul(
                                        [],
                                        group
                                            .window_names
                                            .iter()
                                            .map(|win_name| {
                                                li([], [a([href("#")], [text(&win_name.name)])])
                                            })
                                            .collect::<Vec<Node<Msg>>>(),
                                    ),
                                ],
                            )
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                // BOTTOM-RIGHT: Content 4
                section(
                    [class("window_links_and_window_views")],
                    [
                        header(
                            [],
                            [nav(
                                [class("window_links")],
                                self.window_views
                                    .iter()
                                    .enumerate()
                                    .map(|(index, window)| {
                                        button(
                                            [onclick(move |_| Msg::ActivateWindow(index))],
                                            [text(&window.name)],
                                        )
                                    })
                                    .collect::<Vec<Node<Msg>>>(),
                            )],
                        ),
                        section(
                            [class("window_views")],
                            self.window_views
                                .iter()
                                .enumerate()
                                .map(|(index, window)| {
                                    window
                                        .view()
                                        .map(move |window_msg| Msg::WindowMsg(index, window_msg))
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                    ],
                ),
            ],
        )
    }
}
