mod app;

use app::App;
use gloo_console::log;
// use wasm_bindgen::JsValue;
use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

fn main() {
    // let object = JsValue::from("world");
    log!("Hello Yew");
    //yew::Renderer::<App>::new().render();
    //增加DOM操作
    let div: Element = document().create_element("div").unwrap();
    // Add content, classes etc.
    div.set_inner_html("Hello, World!");
    log!(div);

    let bdiv = document().get_element_by_id("lg").unwrap();
    log!(bdiv.clone());
    yew::Renderer::<App>::with_root(bdiv).render();
}
