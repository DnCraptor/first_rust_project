use std::env;

// mod rest;
mod keys;
mod wss;
mod cron;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect(); // args_os ?
    dbg!(args); // TODO: tune up paths to cert/keys/etc...

    cron::init().await;
    keys::init().unwrap();
    keys::test();
    let future = wss::init();
    future.await;
    // rest::init();
}
