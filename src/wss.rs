// + Cargo.toml
use futures::{StreamExt, FutureExt}; // futures = "*"
use warp::Filter; // warp = {version="*", features = ["tls"]}

pub async fn init() {
    let cert_path = "cert.pem";
    let ppk_path = "key.rsa";
    let puk_path = "key.rsa.pub";
    match files::read_file(std::path::Path::new(cert_path)) {
        Ok(r) => {
            let s = std::str::from_utf8(r.as_slice()).unwrap();
            println!("{}", s);
            ()
        },
        Err(_) => {
            println!("'{}' not found. New self signed cert generation...", cert_path);
            extern crate rcgen; // rcgen = "*"
            use rcgen::generate_simple_self_signed;
            let subject_alt_names = vec!["self signed cert".to_string(), "any".to_string()];
            let cert = generate_simple_self_signed(subject_alt_names).unwrap();
            let cert_pem = cert.serialize_pem().unwrap();
            files::write_file(std::path::Path::new(cert_path), cert_pem.as_bytes()).unwrap();
            println!("{}", cert_pem);
            let ppk = cert.serialize_private_key_pem();
            files::write_file(std::path::Path::new(ppk_path), ppk.as_bytes()).unwrap();
            println!("{}", ppk);
            let puk = cert.get_key_pair().public_key_pem();
            files::write_file(std::path::Path::new(puk_path), puk.as_bytes()).unwrap();
            println!("{}", puk);
            ()
        }
    };

    let echo = warp::path("echo")
    .and(warp::ws())
    .map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| {
            let (tx, rx) = websocket.split();
            println!("websocket tx: {:?}", tx);
            rx
                .forward(tx/* TODO: ??? */)
                .map(|result| {
                    match result {
                        Err(e) => { eprintln!("websocket error: {:?}", e)} ,
                        Ok(_) => {  }
                    }
                })
        })
    });

    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let routes = warp::get().and(echo.or(warp::fs::dir(current_dir)));
    warp::serve(routes)
       .tls()
       .cert_path(cert_path)
       .key_path(ppk_path)
//       .run(([0, 0, 0, 0], 9231)).await;
     .bind(([0, 0, 0, 0], 9231)).await;
    }
