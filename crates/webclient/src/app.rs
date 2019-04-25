use sauron::html::{events::*, *};
use sauron_vdom::Node as VNode;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{self, prelude::*, JsCast};

use crate::Component;

pub use datawindow::DataWindow;
pub use field::Field;
pub use tab::Tab;

mod datawindow;
mod field;
mod tab;

use sauron::Node;

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    Tick,
    DataWindowMsg(datawindow::Msg),
}

pub struct App {
    datawindow: DataWindow,
    click_count: u32,
}

impl App {
    pub fn new() -> App {
        App {
            datawindow: DataWindow::new(),
            click_count: 0,
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::DataWindowMsg(dw_msg) => self.datawindow.update(dw_msg),
            Msg::Tick => sauron::log("ticking"),
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                h1([], [text("Diwata")]),
                button(
                    [onclick(move |_| Msg::Click)],
                    [text(format!("Clicked {}", self.click_count))],
                ),
                div([], [self.datawindow.view().map(Msg::DataWindowMsg)]),
            ],
        )
    }
}
