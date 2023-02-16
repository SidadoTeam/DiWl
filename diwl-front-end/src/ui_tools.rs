use crate::popup_window::*;
use gloo::utils::document;
use web_sys::{Element, Node};

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
