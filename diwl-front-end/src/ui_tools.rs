use crate::popup_window::*;
use gloo::utils::document;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlElement, Node};

//添加和移除弹出框

pub fn add_popup(element_id: &str) {
    let div = document().get_element_by_id(element_id);
    if div.is_none() {
        create_element(element_id);
    }
    _add_popup(element_id);
}

pub fn _add_popup(element_id: &str) {
    let div = document().get_element_by_id(element_id);
    if div.is_some() {
        let div = div.unwrap();
        yew::Renderer::<PopupWindow>::with_root_and_props(
            div,
            PopProps {
                selected_word: String::new(),
            },
        )
        .render();
    }
}

pub fn remove_popup(element_id: &str) {
    let div = document().get_element_by_id(element_id);
    if div.is_some() {
        div.unwrap().remove();
    }
}

pub fn create_element(element_id: &str) {
    let div: Element = document().create_element("div").unwrap();
    // Add content, classes etc.
    div.set_id(element_id);
    let node: Node = div.into();
    let _ = document().body().unwrap().append_child(&node);
}

pub fn add_event_listener() {
    let a = Closure::<dyn FnMut()>::new(move || {
        add_popup("m_popup");
    });
    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let e = caption.get_with_index(0).unwrap();
    let ee = e.parent_element().unwrap();
    ee.dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}
