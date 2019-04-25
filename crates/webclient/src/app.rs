use sauron::html::events::*;
use sauron::html::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use store::{Msg, Store};
use crate::Component;
use crate::Node;

pub use datawindow::DataWindow;
pub use field::Field;
pub use tab::Tab;

mod datawindow;
mod field;
pub mod store;
mod tab;

pub struct App {
    pub store: Rc<RefCell<Store>>,
    datawindow: DataWindow,
}

impl App {
    pub fn new() -> App {
        let count = 0;
        let store = Rc::new(RefCell::new(Store::new(count)));
        let store_clone = Rc::clone(&store);

        let clock = Closure::wrap(
            Box::new(move || store_clone.borrow_mut().msg(&Msg::Tick)) as Box<dyn Fn()>
        );
        sauron::window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                clock.as_ref().unchecked_ref(),
                17,
            )
            .expect("unable to call set_interval with callback");
        clock.forget();
        App {
            store,
            datawindow: DataWindow::new(),
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node {
        let store_clone = Rc::clone(&self.store);
        let clicks = self.store.borrow().click_count();
        div(
            [],
            [
                h1([], [text("Diwata")]),
                button(
                    [onclick(move |_|Msg::Click)],
                    [text(format!("Clicked {}", clicks))],
                ),
                div([], [self.datawindow.view()]),
            ],
        )
    }
}
