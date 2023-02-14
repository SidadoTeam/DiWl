use crate::data::*;
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
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
            <p>
            <button onclick={Callback::from( |_| {

                // 追加单词
                //The night gave me black eyes, but I used to look for light
                //let w = String::from("The night gave me black eyes, but I used to look for light").split(" ");
                //w.into_iter();
                spawn_local(async{
                    //getw_common_all().await;
                    log!(getw_common_len());
                });
            })}>
                { "Click me!" }
            </button>
            </p>
        </div>
    }
}
