use crate::{data::*, ui_style::get_app_style, ui_tools::remove_popup};
use gloo::{events::EventListener, utils::document};
use gloo_console::log;
use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopProps {
    pub selected_word: String,
}

#[function_component(PopupWindow)]
pub fn popup(props: &PopProps) -> Html {
    //需要动态设置窗口的位置?
    //传入当前点击的单词到popup中 默认选择第一个
    //传入被选中的单词
    //state中记录选中单词的index
    //读取div 获得单词列表
    let word_list = get_word_list();
    let f_word = word_list.get(0).unwrap().to_string();
    let selected_word = use_state(|| f_word);

    if !props.selected_word.is_empty() {
        for word in word_list.iter() {
            if word == &props.selected_word {
                selected_word.set(word.clone());
                break;
            }
        }
    }
    let (word_record, word_index) = query_word_record(&*selected_word);

    let fn_ignore = fn_btn_callback(
        word_record.clone(),
        selected_word.clone(),
        word_index,
        "0".to_string(),
    );

    let fn_pickup = fn_btn_callback(
        word_record.clone(),
        selected_word.clone(),
        word_index,
        "3".to_string(),
    );

    let mut word_html_list = Vec::new();
    for name in word_list {
        let _name = name.clone();
        let word_onclick = {
            let state = selected_word.clone();
            Callback::from(move |_: MouseEvent| {
                state.set(_name.clone());
            })
        };
        let mut css = String::new();
        if name == *selected_word {
            css = get_app_style() + " button bg-color-gray padding-word";
        } else {
            css = get_app_style() + " button padding-word";
        }
        let word_html = html! {<span onclick={word_onclick} class={css} >{name}</span>};
        word_html_list.push(word_html);
    }

    html! {
        // <div class ={String::from(sty.get_class_name())}>
        <div class={get_app_style()} >
        <div class ={get_app_style()+ " mcard opx-90 "} >
            <button class={get_app_style() + " button color-white-1 font-size-15"} onclick={Callback::from( |e:MouseEvent| {
                remove_popup("m_popup");
            })}>
                { "Close" }
            </button>
            <hr style="FILTER: alpha(opacity=0,finishopacity=100,style=1)" width="95%" color="gray" SIZE=1 />
            <div style="margin-top:15px;word-break: break-all;word-wrap:break-word;white-space:normal;" >
            {word_html_list}
            </div>

            <div class={get_app_style() + " word-card"}>
                //显示选中的单词
                <div style="font-size:24px;margin-top:1px;">{selected_word.to_string()}</div>

                //显示单词意思
                if word_record.is_some(){
                    <div style = "margin-top:10px">{word_record.clone().unwrap().mean.clone()}</div>
                    <div style = "margin-top:5px">{"short: "}{get_short_mean(&word_record.clone().unwrap().mean.clone())}</div>
                    <div style = "margin-top:5px;font-weight:500" class ={get_app_style()+" color-yell"}>{"level: "}{word_record.unwrap().level.to_string()}</div>
                }else{
                    <div style = "margin-top:10px;color:gray;">{"unknown word!"}</div>
                }

                <p style="margin-top:5px;" class={get_app_style() + " align-buttom-right"}>
                    <button onclick={fn_ignore} class={get_app_style() + " button diwl-button bg-color-black"}>{"Ignore this"}</button>
                    <span style="margin-left:20px" />
                    <button onclick={fn_pickup} class={get_app_style() +" button diwl-button"}>{"Pick up"}</button>
                </p>
            </div>
            </div>
        </div>
    }
}

fn fn_btn_callback(
    _word_record: Option<WordRecord>,
    state: UseStateHandle<String>,
    word_index: i32,
    input_level: String,
) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if _word_record.is_none() {
            return;
        }
        let _state = state.clone();
        let _word_record = _word_record.clone();
        let input_level = input_level.clone();
        spawn_local(async move {
            let word_record = _word_record.unwrap();
            let w = word_record.word.clone();
            if word_index >= 0 {
                user_word_update(word_record.word, word_record.mean, input_level, word_index).await;
            } else {
                user_word_in(word_record.word, word_record.mean, input_level).await;
            }
            getw_user_all().await;
            _state.set(w);
        });
    })
}

fn get_word_list() -> Vec<String> {
    let caption = document().get_elements_by_class_name("ytp-caption-segment");
    let mut words = Vec::new();
    for i in 0..caption.length() {
        let c = caption.get_with_index(i);
        if c.is_some() {
            let cc = c.unwrap();
            let childs = cc.child_nodes();
            let index = usize::try_from(i).unwrap();

            for ii in 0..childs.length() {
                let child = childs.get(ii).unwrap();
                //是否包含<nobr>标签
                if child.has_child_nodes() {
                    //得到里面的字符串
                    //log!("has_child_nodes:", child.first_child());
                    let _child = child.first_child();
                    if _child.is_some() {
                        let _val = _child.unwrap().node_value().unwrap_or_default();
                        let val = _val.trim();
                        if !val.is_empty() {
                            //log!("has_child_nodes:", val);
                            let vals: Vec<String> =
                                val.split(" ").map(|e| String::from(e)).collect();
                            //去除标记
                            let _vals: Vec<String> = vals
                                .iter()
                                .map(|v| {
                                    let mut tmp = v.split("(");
                                    String::from(tmp.next().unwrap())
                                })
                                .collect();
                            words.append(&mut _vals.clone());
                        }
                    }
                } else {
                    //log!("no has_child_nodes:", &child);
                    let _val = child.node_value().unwrap_or_default();
                    let val = _val.trim();
                    if !val.is_empty() {
                        //log!("no has_child_nodes:", val);
                        let vals: Vec<String> = val.split(" ").map(|e| String::from(e)).collect();
                        let _vals: Vec<String> = vals
                            .iter()
                            .map(|v| {
                                let mut tmp = v.split("(");
                                String::from(tmp.next().unwrap())
                            })
                            .collect();
                        words.append(&mut _vals.clone());
                    }
                }
            }
        }
    }
    words
}
