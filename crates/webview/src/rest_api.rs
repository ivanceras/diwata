use crate::app::{App, Msg};
use sauron::{Cmd, Http};

pub fn fetch_window_list() -> Cmd<App, Msg> {
    let url = "http://localhost:8000/windows";
    let text_decoder = |v: String| ron::de::from_str(&v).expect("Unable to decode ron data");
    Http::fetch_with_text_response_decoder(url, text_decoder, Msg::FetchWindowList)
}

pub fn execute_sql_query(sql: String) -> Cmd<App, Msg> {
    let url = format!("http://localhost:8000/sql/?sql={}", sql);
    let text_decoder = |v: String| ron::de::from_str(&v).expect("Expecting row in ron format");
    Http::fetch_with_text_response_decoder(&url, text_decoder, Msg::ReceivedWindowQueryResult)
}
