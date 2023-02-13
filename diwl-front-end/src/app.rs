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
            <button onclick={Callback::from( |_| {
                spawn_local(async{
                    //getw_common_all().await;
                    log!(getw_common_len());
                });
            })}>
                { "Click me!" }
            </button>
        </div>
    }
}
