// + Cargo.toml
use warp::Filter; // warp = {version="*", features = ["tls"]}
use persy::{Persy, Config}; // persy = "*"

mod keygen;
mod rest;
mod wss;

pub async fn init() {
    let cert_path = "cert.pem";
    let ppk_path = "key.rsa";
    let puk_path = "key.rsa.pub";
    match lib::read_file(std::path::Path::new(cert_path)) {
        Ok(r) => {
            let s = std::str::from_utf8(r.as_slice()).unwrap();
            println!("{}", s);
            ()
        }
        Err(_) => {
            println!(
                "'{}' not found. New self signed cert generation...",
                cert_path
            );
            keygen::new_cert_and_pair(cert_path, ppk_path, puk_path);
        }
    };

    /* wss://localhost:9231/path */
    let echo = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| wss::handler(ws));
    let ws = warp::get().and(echo);

// TODO: separate mod?
    match Persy::create("./storage.persy") {
        Ok(_) => { },
        Err(_) => { /* ignore? */ }
    }
    let persy = Persy::open("./storage.persy", Config::new()).unwrap();
    if !persy.exists_segment("seg").unwrap() {
        let mut tx = persy.begin().unwrap();
        tx.create_segment("seg").unwrap();
        let prepared = tx.prepare().unwrap();
        prepared.commit().unwrap();
    }


    let store_filter = warp::any().map(move || persy.clone());

    // POST https://localhost:9231/v1/items -d '[]'
    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("items"))
        .and(warp::path::end())
        .and(rest::json_body())
        .and(store_filter.clone())
        .and_then(rest::add_items_list);

    // GET https://localhost:9231/v1/items
    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("items"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(rest::get_store_as_json);

    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let other = warp::fs::dir(current_dir) /* https://localhost:9231/index.html*/;
    let routes = get_items.or(add_items).or(ws).or(other);
    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(ppk_path)
        //       .run(([0, 0, 0, 0], 9231)).await;
        .bind(([0, 0, 0, 0], 9231))
        .await;
}
