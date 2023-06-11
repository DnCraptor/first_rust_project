// + Cargo.toml
use futures::{StreamExt, FutureExt}; // futures = "*"
use warp::{Reply, ws::Ws}; // warp = {version="*", features = ["tls"]}

pub fn handler(ws: Ws) -> impl Reply {
    ws.on_upgrade(|websocket| {
        let (tx, rx) = websocket.split();
        dbg!(&tx);
        rx
            .forward(tx/* TODO: ??? */)
            .map(|result| {
                match result {
                    Err(e) => { eprintln!("websocket error: {:?}", e)} ,
                    Ok(_) => {  }
                }
            })
    })
}
