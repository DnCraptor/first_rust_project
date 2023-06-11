use std::env;

mod keys;
mod server;
mod cron;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect(); // args_os ?
    dbg!(args); // TODO: tune up paths to cert/keys/etc...

    cron::init().await;
    keys::init().unwrap();
    keys::test();
    let future = server::init();
    future.await;
}
