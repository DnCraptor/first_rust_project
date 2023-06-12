// + Cargo.toml
use warp::{Filter, Reply, Rejection, body, reply}; // warp = {version="*", features = ["tls"]}
use lib::SignedMessageDto;

// GET
pub async fn get_store_as_json(store: super::db::KVDB) -> Result<impl Reply, Rejection> {
    let r = store.get_all();
    let mut res = Vec::new();
    for ele in r {
        res.push(ele.to_dto());
    }
    Ok(reply::json(&res))
}

// POST
pub fn json_body() -> impl Filter<Extract = (SignedMessageDto,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body (and to reject huge payloads)...
    body::content_length_limit(1024 * 32).and(body::json())
}
pub async fn add_item(
    item_dto: SignedMessageDto,
    store: super::db::KVDB
    ) -> Result<impl Reply, Rejection> {
        let item = item_dto.to_native();
        item.verification().unwrap();
        store.upsert(&item);
        Ok(reply::json(&item_dto))
}
