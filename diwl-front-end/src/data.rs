use gloo_console::log;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub static mut wlist_common: Vec<WordRecord> = Vec::new();
pub static mut wlist_user: Vec<WordRecord> = Vec::new();
pub static max_cut_length: usize = 5;
pub static max_level: usize = 2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn getwCommon(i: i32, j: i32) -> JsValue;

    #[wasm_bindgen]
    fn getwUser(i: i32, j: i32) -> JsValue;

    #[wasm_bindgen]
    fn userWordIn(word: String, mean: String, level: String) -> JsValue;

    #[wasm_bindgen]
    fn userWordUpdate(word: String, mean: String, level: String, id: i32) -> JsValue;

    #[wasm_bindgen]
    pub fn openMpopup();
}

pub async fn user_word_in(word: String, mean: String, level: String) {
    let x = userWordIn(word, mean, level);
    let promise = js_sys::Promise::resolve(&x);
    JsFuture::from(promise).await.unwrap_or_default();
}

pub async fn user_word_update(word: String, mean: String, level: String, id: i32) {
    let x = userWordUpdate(word, mean, level, id);
    let promise = js_sys::Promise::resolve(&x);
    JsFuture::from(promise).await.unwrap_or_default();
}

async fn getw_user(start_i: i32, page_size: i32) -> bool {
    let x = getwUser(start_i, page_size);
    let promise = js_sys::Promise::resolve(&x);
    let res = JsFuture::from(promise).await.unwrap_or_default();
    let mut ress: Vec<WordRecord> = from_value(res).unwrap_or_default();
    let ret = ress.len() == 0;
    unsafe {
        wlist_user.append(&mut ress);
    }
    ret
}

pub fn getw_user_len() -> usize {
    unsafe { wlist_user.len() }
}

pub fn getw_common_len() -> usize {
    unsafe { wlist_common.len() }
}

pub async fn getw_user_all() {
    //todo 等待api初始化完成
    unsafe {
        wlist_user.clear();
    }
    for i in 1..4000 / 200 {
        if getw_user(i, 200).await {
            break;
        }
    }
}

pub async fn getw_common_all() {
    unsafe {
        wlist_common.clear();
    }
    //todo 等待api初始化完成
    for i in 1..4000 / 200 {
        if getw_common(i, 200).await {
            break;
        }
    }
}

async fn getw_common(start_i: i32, page_size: i32) -> bool {
    let x = getwCommon(start_i, page_size);
    let promise = js_sys::Promise::resolve(&x);
    let res = JsFuture::from(promise).await.unwrap_or_default();
    let mut ress: Vec<WordRecord> = from_value(res).unwrap_or_default();
    let ret = ress.len() == 0;
    unsafe {
        wlist_common.append(&mut ress);
    }
    ret
}

pub fn query_word(in_word: &str) -> Option<String> {
    unsafe {
        //先在用户词典里查找
        let mut w = wlist_user.iter().find(|w| w.word == in_word);
        if w.is_none() {
            w = wlist_common.iter().find(|w| w.word == in_word);
        }
        if w.is_none() {
            return None;
        }
        let w = w.unwrap();
        if w.level >= max_level.to_string() {
            let mean = get_short_mean(&w.mean);
            return Some(mean);
        }
    }
    None
}

pub fn query_word_record(in_word: &str) -> (Option<WordRecord>, i32) {
    unsafe {
        log!("query_word_record", in_word);
        let ww = wlist_user.iter().enumerate().find(|w| w.1.word == in_word);
        if ww.is_some() {
            log!("find word in word list", in_word);
            return (Some(ww.unwrap().1.clone()), ww.unwrap().0 as i32);
        }

        let w = wlist_common.iter().find(|w| w.word == in_word);
        if w.is_some() {
            return (Some(w.unwrap().clone()), -1);
        }
        return (None, -1);
    }
}

pub fn get_short_mean(in_str: &str) -> String {
    //n.农作物
    //vt.压碎，碾碎；镇压，压倒
    //n.&v.哭
    //vi./vt.(使)结成凝乳；变成凝乳状,curdle
    let re = Regex::new(r#"[\u4e00-\u9fa5]"#).unwrap();
    let tmp = in_str.split(".");
    for ww in tmp.into_iter() {
        let cmp = re.captures(ww);
        if cmp.is_some() {
            //先使用； 在使用，分隔
            let w = get_first_cn("；", ww);
            let w = get_first_cn("，", &w);
            return cut_str(&w, max_cut_length);
        }
    }
    String::from(in_str)
}

fn get_first_cn(split: &str, in_str: &str) -> String {
    let mut tmp = in_str.split(split);
    String::from(tmp.next().unwrap())
}

fn cut_str(in_str: &str, cut_size: usize) -> String {
    let text_vec = in_str.chars().collect::<Vec<_>>();
    if text_vec.len() > cut_size {
        let ss = text_vec[0..cut_size].iter().cloned().collect::<String>();
        return format!("{}...", ss);
    }
    in_str.to_string()
}

pub fn test_short_mean_all() {
    unsafe {
        for w in wlist_common.iter() {
            let mean = get_short_mean(&w.mean);
            let len = mean.chars().count();
            if len > max_cut_length {
                log!(mean, ":", len);
            }
        }
    }
}

#[warn(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordRecord {
    pub word: String,
    pub level: String, //分级
    pub mean: String,  //解释
    pub hitCount: String,
    pub tag: String,
    pub nfts: Vec<String>,
}
