use crate::data::WindowData;
use data_table::DataRow;
use diwata_intel::Window;
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
}

pub struct App {
    window_views: Vec<WindowView>,
    active_window: usize,
}

impl App {
    pub fn new(windows: Vec<Window>) -> App {
        let mut app = App {
            window_views: windows.into_iter().map(WindowView::new).collect(),
            active_window: 0,
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
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ActivateWindow(index) => self.activate_window(index),
            Msg::WindowMsg(index, window_msg) => self.window_views[index].update(window_msg),
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [class("app")],
            [
                header(
                    [],
                    [
                        h1([], [text("Diwata")]),
                        nav(
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
                        ),
                    ],
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
        )
    }
}
