use crate::app::{App, Msg};
use diwata_intel::{data_container::QueryResult, Dao, RecordDetail, TableName};
use sauron::{Cmd, Http};
use wasm_bindgen::JsValue;

pub fn execute_sql_query<F>(sql: &str, msg_receiver: F) -> Cmd<App, Msg>
where
    F: Fn(Result<QueryResult, JsValue>) -> Msg + Clone + 'static,
{
    let url = format!("/sql/?sql={}", sql);
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
    let url = format!("/main_data/{}/", table_name.complete_name(),);
    let text_decoder = |v: String| ron::de::from_str(&v).expect("Unable to decode ron data");
    Http::fetch_with_text_response_decoder(&url, text_decoder, msg_receiver)
}

pub fn retrieve_detail_for_main_tab<F>(
    table: &TableName,
    dao: &Dao,
    msg_receiver: F,
) -> Cmd<App, Msg>
where
    F: Fn(Result<RecordDetail, JsValue>) -> Msg + Clone + 'static,
{
    sauron::log!("dao: {:#?}", dao);
    let dao_string = ron::ser::to_string(dao).expect("Unable to serialize dao");
    sauron::log!("dao_string: {}", dao_string);
    let url = format!(
        "/record_detail/{}/?dao={}",
        table.complete_name(),
        dao_string
    );
    let text_decoder = |v: String| ron::de::from_str(&v).expect("Unable to decode ron data");
    Http::fetch_with_text_response_decoder(&url, text_decoder, msg_receiver)
}
