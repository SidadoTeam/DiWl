mod app;
mod data;
mod fbutton;
mod marking;
mod popup_window;
mod ui_style;
mod ui_tools;
mod websocket;

use app::App;
use data::*;
use fbutton::FButton;
use gloo_console::log;
use marking::*;
// use wasm_bindgen::JsValue;
use gloo::utils::{document, window};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Node};
use yew::prelude::*;
use crate::websocket::WebsocketService;

use crate::{
    popup_window::{PopProps, PopupWindow},
    ui_style::init_app_style,
    ui_tools::add_event_listener,
};

fn main() {
    // let object = JsValue::from("world");
    log!("Hello Yew 2");
    yew::Renderer::<App>::new().render();
    //增加DOM操作
    let div: Element = document().create_element("div").unwrap();
    // Add content, classes etc.
    div.set_inner_html("Hello, World!");
    log!(div);

    // let bdiv = document().get_element_by_id("lg").unwrap();
    // log!(bdiv.clone());
    // yew::Renderer::<App>::with_root(bdiv).render();
    // log!(document());
    // let fff = window().get("getwCommon").unwrap();
    // log!(fff);
    // log!(window());

    //let x = getwCommon(1, 100);
    //log!(x);

    //let ccc = document.getElementsByClassName('ytp-caption-segment')

    //网页面中添加一个元素 把悬浮按钮插入到页面中

    let div: Element = document().create_element("div").unwrap();
    // Add content, classes etc.
    div.set_id("element_id");
    let node: Node = div.into();

    let _ = document().body().unwrap().append_child(&node);

    //let div = document().get_element_by_id("element_id");
    // if div.is_some() {
    //     let div = div.unwrap();
    //     yew::Renderer::<PopupWindow>::with_root_and_props(
    //         div,
    //         PopProps {
    //             selected_word: String::new(),
    //         },
    //     )
    //     .render();
    // }

    init();
    init_app_style();

    spawn_local(async {
        getw_common_all().await;
        log!("common word list loaded size:", getw_common_len());
        getw_user_all().await;
        log!("word list loaded size:", getw_user_len());
    });

    //let ws = WebsocketService::new();
    //let _ = ws.tx.clone().try_send( "hello".to_string());
    // log!(res);
}
