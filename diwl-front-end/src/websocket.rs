use futures::{channel::mpsc::Sender};
use gloo_console::log;
use reqwasm::websocket::{futures::{WebSocket}, Message};
use futures::{SinkExt, StreamExt};

use wasm_bindgen_futures::spawn_local;

pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                log!("got event from channel! {}", s.clone());
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log!("from websocket: {}", data);
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log!("from websocket: {}", val);
                        }
                    }
                    Err(e) => {
                        // log::error!("ws: {:?}", e)
                    }
                }
            }
            log!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}