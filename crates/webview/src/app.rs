use crate::rest_api;
use diwata_intel::{
    data_container::{AppData, QueryResult, WindowData},
    window::GroupedWindow,
    RecordDetail,
};
use sauron::{
    html::{attributes::*, events::*, *},
    Browser, Component, Node,
};

use wasm_bindgen::JsValue;

use window_list_view::WindowListView;
use window_view::WindowView;

mod column_view;
mod detail_view;
mod field_view;
mod row_view;
mod tab_view;
mod table_view;
mod toolbar_view;
mod window_list_view;
mod window_view;

pub type Cmd = sauron::Cmd<App, Msg>;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Msg {
    ActivateWindow(usize),
    RemoveWindow(usize),
    WindowMsg(usize, window_view::Msg),
    BrowserResized(i32, i32),
    Tick,
    WindowListMsg(window_list_view::Msg),
    FetchWindowList(Result<Vec<GroupedWindow>, JsValue>),
    ReceivedWindowQueryResult(usize, Result<QueryResult, JsValue>),
    ReceivedWindowData(Result<QueryResult, JsValue>),
    ReceivedWindowDataNextPage(usize, usize, Result<QueryResult, JsValue>),
    ReceivedWindowMainTabDetail(usize, usize, Result<RecordDetail, JsValue>),
}

pub struct App {
    window_views: Vec<WindowView>,
    window_data: Vec<WindowData>,
    active_window: usize,
    browser_height: i32,
    browser_width: i32,
    window_list_view: WindowListView,
    is_page_request_in_flight: bool,
}

impl App {
    pub fn new(app_data: AppData, browser_width: i32, browser_height: i32) -> App {
        let mut app = App {
            window_views: app_data
                .windows
                .into_iter()
                .zip(app_data.window_data.iter())
                .map(|(window, window_data)| {
                    WindowView::new(window, &window_data, browser_width, browser_height)
                })
                .collect(),
            window_data: app_data.window_data,
            window_list_view: WindowListView::new(app_data.grouped_window),
            active_window: 0,
            browser_width,
            browser_height,
            is_page_request_in_flight: false,
        };
        app.update_active_window();
        app.update_size_allocation();
        app
    }

    fn update_size_allocation(&mut self) {
        let window_list_size = self.calculate_window_list_size();
        self.window_list_view.set_allocated_size(window_list_size);
    }

    fn calculate_window_list_size(&self) -> (i32, i32) {
        (200, self.browser_height - self.logo_height())
    }

    fn logo_height(&self) -> i32 {
        170
    }

    fn update_active_window(&mut self) {
        let active_window = self.active_window;
        self.window_views
            .iter_mut()
            .enumerate()
            .for_each(|(index, window)| {
                if index == active_window {
                    window.show()
                } else {
                    window.hide()
                }
            })
    }

    fn activate_window(&mut self, index: usize) {
        self.active_window = index;
        self.update_active_window();
    }

    fn activate_last_added_window(&mut self) {
        self.activate_window(self.window_views.len() - 1);
    }

    fn remove_window(&mut self, index: usize) {
        self.window_views.remove(index);
        //TODO: activate the last opened one
        self.activate_window(0);
    }

    fn setup_window_resize_listener(&self) -> Cmd {
        Browser::onresize(Msg::BrowserResized)
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd {
        Cmd::batch(vec![
            //rest_api::fetch_window_list(),
            self.setup_window_resize_listener(),
        ])
    }

    fn update(&mut self, msg: Msg) -> Cmd {
        match msg {
            Msg::ActivateWindow(index) => {
                self.activate_window(index);
                Cmd::none()
            }
            Msg::RemoveWindow(index) => {
                self.remove_window(index);
                Cmd::none()
            }

            Msg::WindowMsg(
                window_index,
                window_view::Msg::MainTabMsg(tab_view::Msg::TableMsg(table_view::Msg::RowMsg(
                    row_index,
                    row_view::Msg::DoubleClick,
                ))),
            ) => {
                sauron::log!("Row {} is dblclicked", row_index);
                let window_msg = window_view::Msg::MainTabMsg(tab_view::Msg::TableMsg(
                    table_view::Msg::RowMsg(row_index, row_view::Msg::DoubleClick),
                ));
                let window_view = &mut self.window_views[window_index];
                window_view.update(window_msg);
                let main_tab_view = &mut window_view.main_tab;
                // show the row first, while the detail is loading
                main_tab_view.show_detail_view(row_index);

                let table_name = &main_tab_view.table_name;
                let dao = &main_tab_view.table_view.row_views[row_index].primary_dao();
                rest_api::retrieve_detail_for_main_tab(table_name, dao, move |detail| {
                    Msg::ReceivedWindowMainTabDetail(window_index, row_index, detail)
                })
            }

            Msg::WindowMsg(index, window_view::Msg::ToolbarMsg(toolbar_view::Msg::RunQuery)) => {
                let sql = self.window_views[index].sql_query();
                if let Some(sql) = sql {
                    sauron::log!("In app.rs Run the query: {}", sql);
                    rest_api::execute_sql_query(&sql, move |window_rows| {
                        Msg::ReceivedWindowQueryResult(index, window_rows)
                    })
                } else {
                    sauron::log!("Nothing to execute!");
                    Cmd::none()
                }
            }
            Msg::WindowMsg(window_index, window_view::Msg::MainTabMsg(tab_msg)) => {
                let main_tab = &mut self.window_views[window_index].main_tab;
                let main_tab_current_page = self.window_data[window_index].main_tab_current_page;
                let next_page = main_tab_current_page + 1;
                main_tab.update(tab_msg);
                sauron::log!(
                    "is a page request in flight: {}",
                    self.is_page_request_in_flight
                );
                if main_tab.need_next_page() && !self.is_page_request_in_flight {
                    self.is_page_request_in_flight = true;
                    sauron::log!(
                        "---->>> is a page request in flight: {}",
                        self.is_page_request_in_flight
                    );
                    rest_api::fetch_window_data_next_page(
                        &main_tab.table_name,
                        next_page,
                        move |query_result| {
                            Msg::ReceivedWindowDataNextPage(window_index, next_page, query_result)
                        },
                    )
                } else {
                    Cmd::none()
                }
            }
            Msg::WindowMsg(index, window_msg) => self.window_views[index].update(window_msg),
            Msg::BrowserResized(width, height) => {
                sauron::log!("Browser is resized to: {}, {}", width, height);
                self.browser_width = width;
                self.browser_height = height;
                //also notify all opened windows with the resize;
                self.window_views.iter_mut().for_each(|window| {
                    window.update(window_view::Msg::BrowserResized(width, height));
                });
                self.update_size_allocation();
                Cmd::none()
            }
            Msg::Tick => {
                sauron::log("Ticking");
                Cmd::none()
            }
            Msg::WindowListMsg(window_list_view::Msg::ClickedWindow(table_name)) => {
                sauron::log!("fetching data for {}", table_name.complete_name());
                let url = format!("/{}", table_name.complete_name());
                sauron::history()
                    .replace_state_with_url(&JsValue::NULL, &table_name.complete_name(), Some(&url))
                    .expect("unable to replace state with url");
                rest_api::fetch_window_data(&table_name, move |window_rows| {
                    Msg::ReceivedWindowData(window_rows)
                })
            }
            Msg::WindowListMsg(window_list_msg) => {
                self.window_list_view.update(window_list_msg);
                Cmd::none()
            }
            Msg::FetchWindowList(Ok(window_list)) => {
                self.window_list_view
                    .update(window_list_view::Msg::ReceiveWindowList(window_list));
                Cmd::none()
            }
            Msg::FetchWindowList(Err(js_value)) => {
                sauron::log!("There was an error fetching window list: {:#?}", js_value);
                Cmd::none()
            }

            Msg::ReceivedWindowData(query_result) => {
                match query_result {
                    Ok(query_result) => {
                        if let Some(window) = query_result.window {
                            let window_clone = window.clone();
                            let sql_query =
                                format!("SELECT * FROM {}", window.table_name().complete_name());
                            let mut window_data = WindowData::from_rows(query_result.rows);
                            window_data.sql_query = Some(sql_query.to_string());
                            let new_window = WindowView::new(
                                window_clone,
                                &window_data,
                                self.browser_width,
                                self.browser_height,
                            );
                            // set the previous sql query
                            // new_window.set_window_data(window_data);
                            // replace the previous window
                            self.window_data.push(window_data);
                            self.window_views.push(new_window);
                            self.activate_last_added_window();
                        } else {
                            sauron::log!("No window returned in query result");
                        }
                        Cmd::none()
                    }
                    Err(err) => {
                        sauron::log!("error fetching window data: {:?}", err);
                        Cmd::none()
                    }
                }
            }
            Msg::ReceivedWindowDataNextPage(window_index, page, Ok(query_result)) => {
                sauron::log!("Got data for next page {}: {:#?}", page, query_result);
                let window_data = &mut self.window_data[window_index];
                window_data.add_main_data_page(query_result.rows);
                window_data.main_tab_current_page = page;
                self.window_views[window_index].set_window_data(window_data);
                self.is_page_request_in_flight = false;
                Cmd::none()
            }
            Msg::ReceivedWindowDataNextPage(_window_index, page, Err(_e)) => {
                sauron::log!("Error retrieving next page {}", page);
                Cmd::none()
            }

            Msg::ReceivedWindowQueryResult(index, Ok(query_result)) => {
                if let Some(window) = query_result.window {
                    let window_clone = window.clone();
                    let window_data = WindowData::from_rows(query_result.rows);
                    //replace the data on this window index
                    let new_window = WindowView::new(
                        window_clone,
                        &window_data,
                        self.browser_width,
                        self.browser_height,
                    );
                    self.window_data[index] = window_data;
                    // set the previous sql query
                    // replace the previous window
                    self.window_views[index] = new_window;
                } else {
                    sauron::log!("No window returned in query result");
                }
                Cmd::none()
            }
            Msg::ReceivedWindowQueryResult(_index, Err(_err)) => {
                sauron::log!("Error retrieveing records from sql query");
                Cmd::none()
            }
            Msg::ReceivedWindowMainTabDetail(window_index, row_index, Ok(record_detail)) => {
                sauron::log!("Got window main tab detail: {:#?}", record_detail);
                let detail_window = record_detail.window.clone();
                let window_data = &mut self.window_data[window_index];
                window_data.set_record_detail(record_detail);
                let mut new_window = WindowView::new(
                    detail_window,
                    &window_data,
                    self.browser_width,
                    self.browser_height,
                );
                sauron::log!("Window data: {:#?}", window_data);
                new_window.show_main_tab_detail_view(row_index);
                new_window.update_size_allocation();
                self.window_views[window_index] = new_window;
                Cmd::none()
            }
            Msg::ReceivedWindowMainTabDetail(_window_index, _row_index, Err(_record_detail)) => {
                sauron::log!("Error retrieveing window main tab detail..");
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            // GRID
            [class("app")],
            [
                section(
                    [class("logo_and_window_list")],
                    [
                        header([class("logo")], []),
                        self.window_list_view.view().map(Msg::WindowListMsg),
                    ],
                ),
                section(
                    [class("window_links_and_window_views")],
                    [
                        header(
                            [class("window_links_and_logout")],
                            [
                                nav(
                                    [class("logout")],
                                    [
                                        button([], [text("logout")]),
                                        button([], [text("Connect to database..")]),
                                    ],
                                ),
                                nav(
                                    [class("window_links")],
                                    self.window_views
                                        .iter()
                                        .enumerate()
                                        .map(|(index, window)| {
                                            a(
                                                [
                                                    class("tab_links"),
                                                    classes_flag([("active", window.is_visible)]),
                                                    onclick(move |_| Msg::ActivateWindow(index)),
                                                ],
                                                [
                                                    text(&window.name),
                                                    button(
                                                        [
                                                            class("window_close_btn"),
                                                            onclick(move |_| {
                                                                Msg::RemoveWindow(index)
                                                            }),
                                                        ],
                                                        [text("x")],
                                                    ),
                                                ],
                                            )
                                        })
                                        .collect::<Vec<Node<Msg>>>(),
                                ),
                            ],
                        ),
                        section(
                            [class("window_views")],
                            self.window_views
                                .iter()
                                .enumerate()
                                .map(|(index, window)| {
                                    window
                                        .view()
                                        .map(move |window_msg| Msg::WindowMsg(index, window_msg))
                                })
                                .collect::<Vec<Node<Msg>>>(),
                        ),
                    ],
                ),
            ],
        )
    }
}
