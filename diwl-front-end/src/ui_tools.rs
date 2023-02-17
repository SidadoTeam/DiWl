use crate::{data::openMpopup, popup_window::*};
use gloo::utils::document;
use gloo_console::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlElement, Node};

//添加和移除弹出框

pub fn add_popup(element_id: &str) {
    let div = document().get_element_by_id(element_id);
    if div.is_none() {
        create_element(element_id);
    }
    // openMpopup();
    _add_popup(element_id);
}

pub fn _add_popup(element_id: &str) {
    let div = document().get_element_by_id(element_id);
    if div.is_some() {
        let div = div.unwrap();
        let _ = div.set_attribute("style", "width: 300px; height:300px");
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
    let div: Element = document().create_element("div").unwrap(); //dialog
                                                                  // Add content, classes etc.
    div.set_id(element_id);
    let node: Node = div.into();
    let _ = document().body().unwrap().append_child(&node);
    // let caption = document().get_element_by_id("ytp-caption-window-container");
    // if caption.is_some() {
    //     let _ = caption.unwrap().append_child(&node);
    // }
    //弹出对话框
}

pub fn add_event_listener() {
    let a = Closure::<dyn FnMut()>::new(move || {
        log!("add_event_listener");
        add_popup("m_popup");
    });
    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let e = caption.get_with_index(0).unwrap();
    let ee = e.parent_element().unwrap();
    let attr = ee.get_attribute("event_added");
    if attr.is_none() {
        ee.dyn_ref::<HtmlElement>()
            .expect("#green-square be an `HtmlElement`")
            .set_onclick(Some(a.as_ref().unchecked_ref()));
        a.forget();
        log!("set_onclick");
        ee.set_attribute("event_added", "added");
    }
}
