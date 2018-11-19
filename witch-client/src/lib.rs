
#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
use yew::prelude::*;

use rustorm::types::SqlType;
use witch::dataview::{
    DataView,
    Field,
};


type Context = ();

pub struct Model {
    dataview: DataView,
}

pub enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let csv = r#"
pl,version,speed,vm,size,compiler
rust,1,fast,false,small,rustc
haskel,1,fast,false,small,ghc
c,99,fast,false,small,clang
java,8,medium,true,large,jdk
        "#;
        let fields = vec![
            Field{
                name: "pl".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "compiler".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "speed".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "vm".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "size".into(),
                sql_type: SqlType::Text,
            },
            Field{
                name: "version".into(),
                sql_type: SqlType::Int,
            },
        ];
        let dataview = DataView::new_from_csv(fields, csv);
        console!(log, format!("dataview: {:#?}", dataview));
        Model { dataview }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}

