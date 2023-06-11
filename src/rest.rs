// + Cargo.toml
extern crate iron; // iron = "*"
extern crate router; // router = "*"

use iron::*;

#[allow(unused_variables)]
pub fn get_records(req: &mut Request) -> IronResult<Response> {
    return Ok(Response::with((status::BadRequest, "todo")));
}

#[allow(unused_variables)]
pub fn delete_record(req: &mut Request) -> IronResult<Response> {
    return Ok(Response::with((status::BadRequest, "todo")));
}

pub fn init() {

    let mut router = router::Router::new();
    {
        router.get(
            "/api/v1/records",
            move |req: &mut Request| get_records(req),
            "rid1"
        );
    }
    {
        router.delete(
            "/api/v1/record/:id",
            move |req: &mut Request| delete_record(req),
            "rid2"
        );
    }
    println!("rest::init");
    Iron::new(router).http("localhost:3000").unwrap();    

}