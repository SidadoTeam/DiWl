use crate::data::*;
use crate::ui_tools::*;
use gloo::utils::document;
use gloo_console::log;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Element, MouseEvent};


#[function_component(FButton)]
pub fn fbutton() -> Html {
    let sty = stylist::style!(
        r#"
        position: fixed;
        height: 30px;
        width: 80px;
        top: 90px;
        right: 50px;
        background: #282a22;
    "#
    )
    .expect("Failed to mount style");

    html! {
        <main>
            <div class={String::from(sty.get_class_name())} >
            <button class="text-white" onclick={Callback::from( |e:MouseEvent| {
            //    log!(e);
            //    let _target = e.target().unwrap();
            //    let target = _target.unchecked_into::<Element>();
            //    log!(target);
                add_popup("m_popup");
            })}>
                { "Click me!" }
            </button>
            </div>
        </main>
    }
}

fn foo() {
    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let c = caption.get_with_index(0);
    if c.is_some() {
        //定时执行 并且判读内容是否发生变化
        let line = c.clone().unwrap().inner_html();
        let y = line.split(" ");
        let r = y
            .into_iter()
            .map(|w| w.to_string() + "() ")
            .reduce(|w, s| w + &s);
        c.unwrap().set_inner_html(&r.unwrap());
    }
}
