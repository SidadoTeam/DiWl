mod app;
mod data;
mod fbutton;
mod marking;

use marking::*;
use app::App;
use data::*;
use fbutton::FButton;
use gloo_console::log;
// use wasm_bindgen::JsValue;
use gloo::utils::{document, window};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Node};
use yew::prelude::*;

fn main() {
    // let object = JsValue::from("world");
    log!("Hello Yew");
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
    div.set_inner_html("Hello, World!");
    div.set_id("element_id");
    let node: Node = div.into();

    let _ = document().body().unwrap().append_child(&node);

    let div = document().get_element_by_id("element_id");
    if div.is_some() {
        let div = div.unwrap();
        yew::Renderer::<FButton>::with_root(div).render();
    }

    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let c = caption.get_with_index(0);
    if c.is_some() {
        log!(c);
    }

    init();

    spawn_local(async {
        getw_common_all().await;
        log!("common word list loaded size:{}", getw_common_len());
    });
}
