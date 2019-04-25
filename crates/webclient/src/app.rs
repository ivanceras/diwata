use sauron::html::events::*;
use sauron::html::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::Component;
use crate::Node;

pub use datawindow::DataWindow;
pub use field::Field;
pub use tab::Tab;

mod datawindow;
mod field;
mod tab;

#[derive(Debug,Clone)]
pub enum Msg {
    Click,
    Tick,
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
        match msg{
            Msg::Click => self.click_count += 1,
            _ => {},
        }
    }

    fn view(&self) -> Node {
        div(
            [],
            [
                h1([], [text("Diwata")]),
                button(
                    [onclick(move |_|Msg::Click)],
                    [text(format!("Clicked {}", self.click_count))],
                ),
                div([], [self.datawindow.view()]),
            ],
        )
    }
}
