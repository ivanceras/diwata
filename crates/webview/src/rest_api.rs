use crate::app::{App, Msg};
use diwata_intel::{data_container::QueryResult, Rows, Window, TableName};
use sauron::{Cmd, Http};
use wasm_bindgen::JsValue;

pub fn fetch_window_list() -> Cmd<App, Msg> {
    let url = "http://localhost:8000/windows";
    let text_decoder = |v: String| ron::de::from_str(&v).expect("Unable to decode ron data");
    Http::fetch_with_text_response_decoder(url, text_decoder, Msg::FetchWindowList)
}

pub fn execute_sql_query<F>(sql: String, msg_receiver: F) -> Cmd<App, Msg>
where
    F: Fn(Result<QueryResult, JsValue>) -> Msg + Clone + 'static,
{
    let url = format!("http://localhost:8000/sql/?sql={}", sql);
    let text_decoder = |v: String| {
        let value = ron::de::from_str(&v);
        match value {
            Ok(value) => value,
            Err(e) => {
                sauron::log!("Error: {}", e);
                panic!();
            }
        }
    };
    Http::fetch_with_text_response_decoder(&url, text_decoder, msg_receiver)
}

pub fn fetch_window_data<F>(table_name: &TableName, msg_receiver: F) -> Cmd<App, Msg>
where
    F: Fn(Result<QueryResult, JsValue>) -> Msg + Clone + 'static,
{
    let sql = format!("SELECT * FROM {} LIMIT 40",table_name.complete_name());
    execute_sql_query(sql, msg_receiver)
}
