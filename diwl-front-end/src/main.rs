mod app;
mod data;

use app::App;
use data::*;
use gloo_console::log;
// use wasm_bindgen::JsValue;
use gloo::utils::{document, window};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
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
    spawn_local(async {
        getw_common_all().await;
        log!("common word list loaded size:{}", getw_common_len());
    });
}
