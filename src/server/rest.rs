// + Cargo.toml
use warp::{Filter, Reply, Rejection, body, reply}; // warp = {version="*", features = ["tls"]}
use persy::Persy; // persy = "*"

// GET
pub async fn get_store_as_json(store: Persy) -> Result<impl Reply, Rejection> {
    let mut r = Vec::new();
    for (_id, content) in store.scan("seg").unwrap() {
        dbg!(_id);
        dbg!(&content);
        for el in content {
            r.push(el as u32);
        }
    }
    Ok(reply::json(&r))
}

// POST
pub fn json_body() -> impl Filter<Extract = (Vec<u8>,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body (and to reject huge payloads)...
    body::content_length_limit(1024 * 32).and(body::json())
}
pub async fn add_items_list(
    items: Vec<u8>,
    store: Persy
    ) -> Result<impl Reply, Rejection> {
        dbg!(&items);
        let mut tx = store.begin().unwrap();
        let _id = tx.insert("seg", &items).unwrap();
        let prepared = tx.prepare().unwrap();
        prepared.commit().unwrap();
        dbg!(_id);
        // TODO:
        Ok(reply::json(&items))
}
