use crate::assets;
use sauron::{
    html::{attributes::*, *},
    Attribute, Node,
};

pub fn new<MSG>(width: i32, height: i32, event: Attribute<MSG>) -> Node<MSG>
where
    MSG: Clone,
{
    div(
        [
            class("search_icon_and_column_filter"),
            styles([("width", px(width)), ("height", px(height))]),
        ],
        [
            div(
                [class("search_icon")],
                [assets::svg_search_icon(18, 18, "#888")],
            ),
            input([r#type("text"), class("column_filter"), event], []),
        ],
    )
}
