// + Cargo.toml
use warp::{Filter, Reply, Rejection, body, reply}; // warp = {version="*", features = ["tls"]}

// GET
pub async fn get_store_as_json(store: super::db::KVDB) -> Result<impl Reply, Rejection> {
    let r = store.get_all();
    Ok(reply::json(&r))
}

// POST
pub fn json_body() -> impl Filter<Extract = (lib::SignedMessage,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body (and to reject huge payloads)...
    body::content_length_limit(1024 * 32).and(body::json())
}
pub async fn add_item(
    item: lib::SignedMessage,
    store: super::db::KVDB
    ) -> Result<impl Reply, Rejection> {
        println!("deserialized = {}", item.to_json().unwrap());
        item.verification().unwrap();
        store.upsert(&item);
        Ok(reply::json(&item))
}
