

//mod rest;
mod keys;
mod wss;

#[tokio::main]
async fn main() {
    //rest::init();
    keys::init().unwrap();
    keys::test();
    let future = wss::init();
    future.await;
}
