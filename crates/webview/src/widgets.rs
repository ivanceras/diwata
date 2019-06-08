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
        [class("search_icon_and_column_filter")],
        [
            div(
                [class("search_icon")],
                [assets::svg_search_icon(18, 18, "#888")],
            ),
            input([r#type("text"), class("column_filter"), event], []),
        ],
    )
}

pub fn quick_find<MSG>(h: i32, event: Attribute<MSG>) -> Node<MSG>
where
    MSG: Clone,
{
    div(
        [class("quick_find_widget")],
        [
            div(
                [class("quick_search_icon")],
                [assets::svg_search_icon(h, h, "#888")],
            ),
            input(
                [
                    r#type("text"),
                    class("quick_search_input"),
                    styles([("height", px(h))]),
                    event,
                ],
                [],
            ),
            button([class("quick_find_btn")], [text("Quick find")]),
        ],
    )
}
