// + Cargo.toml
use warp::Filter; // warp = {version="*", features = ["tls"]}

// GET
pub async fn get_store_as_json(store: lib::Store) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.u32_v.as_slice();
    dbg!(r);
    Ok(warp::reply::json(&r))
}

// POST
pub fn json_body() -> impl Filter<Extract = (Vec<u32>,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
pub async fn add_items_list(
    items: Vec<u32>,
    store: lib::Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        dbg!(&items);
        let mut r = store.u32_v;
        for ele in items {
            r.push(ele);
        }
        dbg!(&r);
        Ok(warp::reply::json(&r))
}
