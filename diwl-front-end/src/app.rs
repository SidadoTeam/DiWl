use crate::data::*;
use gloo::utils::document;
use gloo_console::log;
use regex::Regex;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle text-red-500">{ "from Yew with " }<i class="heart" /></span>
            <div style = "margin-top:10px" />
            <p class = "ytp-caption-segment">{"Simulated YouTube captioning"}</p>
            <p class = "ytp-caption-segment">{"Hello welcome"}</p>
            <div style = "margin-top:10px" />

            <p class = "test-div"><nobr>{"hello"}</nobr>{"Simulated YouTube captioning"}</p>

            <div>{"onclick"}</div>
            <div style = "margin-top:10px" />

            <p>
            <button onclick={Callback::from( |_| {
                // 追加单词
                //The night gave me black eyes, but I used to look for light
                //let w = String::from("The night gave me black eyes, but I used to look for light").split(" ");
                //w.into_iter();
                // spawn_local(async{
                //     //getw_common_all().await;
                //     log!(getw_common_len());
                // });
                // test_find(); 1,loneliness 2,curtail
                let w = query_word("loneliness").unwrap_or_default();
                log!(w);
                // test_short_mean_all();
            })}>
                { "TestComp" }
            </button>
            </p>
        </div>
    }
}

fn test_find() {
    let caption = document().get_elements_by_class_name("test-div");
    let div = caption.get_with_index(0).unwrap();
    log!("div child_nodes len:", div.child_nodes().length());
    log!("div children len:", div.children().length());

    let child_node2 = div.child_nodes().get(0).unwrap();
    log!("child_node2:", &child_node2);
    //let node_value = child_node2.node_value().unwrap();
    //log!("node value:", node_value);
    let props = Props {
        list: vec!["hello1".to_string(), "hello2".to_string()],
    };
    yew::Renderer::<TestComp>::with_root_and_props(div, props).render();
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub list: Vec<String>,
}

#[function_component(TestComp)]
fn test_html(props: &Props) -> Html {
    html! {
        <nobr>
            {
                props.list.clone().into_iter().map(|name| {
                    html!{<p key = {name.clone()} onclick={Callback::from(|e:MouseEvent| {
                        let target = e.target().unwrap();
                        log!(target.unchecked_into::<Element>().inner_html());
                    })} >{name}</p>}
                }).collect::<Html>()
            }
        </nobr>
    }
}
