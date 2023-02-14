use std::fs::OpenOptions;

use crate::data::*;
use futures_util::stream::StreamExt;
use gloo::utils::document;
use gloo_console::log;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlCollection, MouseEvent};
use yew::{function_component, html, Callback, Html, Properties};

pub fn init() {
    // if getw_common_len() == 0 {
    //     return;
    // }
    //启动定时器
    spawn_local(async {
        let mut last_lines: Option<[String; 4]> = None;
        // loop {
        //     // log!("hello");
        //     last_lines = process(last_lines);
        //     TimeoutFuture::new(1000).await;
        // }
        for i in 0..500 {
            last_lines = process(last_lines);
            TimeoutFuture::new(1000).await;
        }
    });
}

fn process(_last_lines: Option<[String; 4]>) -> Option<[String; 4]> {
    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let mut last_lines = [String::new(), String::new(), String::new(), String::new()];
    if caption.length() > 4 {
        return _last_lines;
    }
    for i in 0..caption.length() {
        let c = caption.get_with_index(i);
        if c.is_some() {
            let cc = c.unwrap();
            let childs = cc.child_nodes();
            let mut current_line: Vec<String> = Vec::new();
            let index = usize::try_from(i).unwrap();

            for ii in 0..childs.length() {
                let child = childs.get(ii).unwrap();
                //是否包含<nobr>标签
                if child.has_child_nodes() {
                    //得到里面的字符串
                    //log!("has_child_nodes:", child.first_child());
                    let _val = child.first_child().unwrap().node_value().unwrap();
                    let val = _val.trim();
                    if !val.is_empty() {
                        //log!("has_child_nodes:", val);
                        let mut vals = val.split(" ").map(|e| String::from(e)).collect();
                        current_line.append(&mut vals);
                    }
                } else {
                    //log!("no has_child_nodes:", &child);
                    let _val = child.node_value().unwrap_or_default();
                    let val = _val.trim();
                    if !val.is_empty() {
                        //log!("no has_child_nodes:", val);
                        let mut vals = val.split(" ").map(|e| String::from(e)).collect();
                        current_line.append(&mut vals);
                    }
                }
            }

            //log!("current line:", &line);
            let __last_lines = _last_lines.clone().unwrap_or_default();
            let last_line = &__last_lines[index];
            //log!("last_line line:", last_line);
            let last_ll: Vec<&str> = last_line.split(" ").collect();
            let mut has_change = false;
            let mut res_line: Vec<String> = Vec::new();
            let mut pure_res_line = String::new();
            for (x, word) in current_line.iter().enumerate() {
                let last_word = last_ll.get(x);
                let mut _word = String::from(word.clone());
                let mut pure_word = String::from(word.clone());
                if last_word.is_none() || (last_word.is_some() && last_word.unwrap() != word) {
                    // log!("word:", word.clone());
                    // if last_word.is_some(){
                    //     log!("last_word:", last_word.unwrap().clone());
                    // }
                    // todo 添加回调函数
                    //查询单词列表 获取解释
                    //添加点击事件
                    pure_word = pure_word + "()";
                    // _word = "<nobr onclick = \"console.log('".to_string()
                    //     + &pure_word
                    //     + "')\">"
                    //     + &pure_word
                    //     + "</nobr>";
                    has_change = true;
                }

                //res_line = res_line + " " + &_word;
                let _pure_word = String::new() + &pure_word + " ";
                res_line.push(String::from(&_pure_word));
                pure_res_line = pure_res_line + " " + &pure_word;
            }
            //res_line = res_line.trim().to_owned();
            pure_res_line = pure_res_line.trim().to_owned();
            if has_change && !res_line.is_empty() {
                // cc.set_inner_html(&res_line);
                //log!("render: res_line", res_line.len());
                let props = Props { list: res_line };
                yew::Renderer::<CaptionComp>::with_root_and_props(cc, props).render();
            }
            last_lines[index] = pure_res_line;
        }
    }
    Some(last_lines)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub list: Vec<String>,
}

#[function_component(CaptionComp)]
fn caption_comp(props: &Props) -> Html {
    html! {
        {
            props.list.clone().into_iter().map(|name| {
                html!{<nobr key = {name.clone()} onclick={Callback::from(|e:MouseEvent| {
                    let target = e.target().unwrap();
                    log!(target.unchecked_into::<Element>().inner_html());
                })} >{name}</nobr>}
            }).collect::<Html>()
        }
    }
}
