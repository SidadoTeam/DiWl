use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub static mut wlist_common: Vec<WordRecord> = Vec::new();
pub static mut wlist_know: Option<Vec<WordRecord>> = None;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn getwCommon(i: i32, j: i32) -> JsValue;
}

pub fn getw_common_len() -> usize {
    unsafe { wlist_common.len() }
}

pub async fn getw_common_all() {
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

#[derive(Serialize, Deserialize)]
pub struct WordRecord {
    word: String,
    level: String, //分级
    mean: String,  //解释
    hitCount: String,
    tag: String,
    nfts: Vec<String>,
}
