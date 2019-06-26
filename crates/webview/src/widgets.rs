use crate::assets;
use sauron::{
    html::{attributes::*, units::*, *},
    Attribute, Node,
};

pub fn search_widget<MSG>(event: Attribute<MSG>) -> Node<MSG>
where
    MSG: Clone,
{
    div(
        vec![class("search_icon_and_column_filter")],
        vec![
            div(
                vec![class("search_icon")],
                vec![assets::svg_search_icon(18, 18, "#888")],
            ),
            input(vec![r#type("text"), class("column_filter"), event], vec![]),
        ],
    )
}

pub fn quick_find<MSG>(h: i32, event: Attribute<MSG>) -> Node<MSG>
where
    MSG: Clone,
{
    div(
        vec![class("quick_find_widget")],
        vec![
            div(
                vec![class("quick_search_icon")],
                vec![assets::svg_search_icon(h, h, "#888")],
            ),
            input(
                vec![
                    r#type("text"),
                    class("quick_search_input"),
                    styles(vec![("height", px(h))]),
                    event,
                ],
                vec![],
            ),
            button(vec![class("quick_find_btn")], vec![text("Quick find")]),
        ],
    )
}
