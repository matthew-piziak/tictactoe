extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use iron::status;
use urlencoded::UrlEncodedQuery;

use std::env;

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let params = req.get_ref::<UrlEncodedQuery>().unwrap();
    Ok(Response::with((status::Ok, format!("Board: {:?}!", params["board"]))))
}

// Run the Tic Tac Toe server.
fn main() {
    // Set up routing.
    let mut router: Router = Router::new();
    router.get("/", root_handler, "index");

    // Run the server.
    let port_str = env::var("PORT").unwrap_or(String::new());
    let port = port_str.parse().unwrap_or(8080);
    Iron::new(router).http(("0.0.0.0", port)).expect("Could not initialize server");
}
