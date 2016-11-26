//! A Tic-tac-toe server.

#![feature(try_from)]
#![deny(missing_docs)]

extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use iron::status;
use urlencoded::UrlEncodedQuery;

use std::env;
use std::convert::TryFrom;

mod board;

fn root_handler(req: &mut Request) -> IronResult<Response> {
    // TODO: to response
    let params = req.get_ref::<UrlEncodedQuery>().expect("Could not read query parameters");
    let board_param: String =
        params["board"].first().expect("Could not read `board` query parameter").clone();
    match board::Board::try_from(board_param) {
        Ok(board) => Ok(Response::with((status::Ok, format!("{}\n", board::play(board))))),
        Err(()) => Ok(Response::with((status::BadRequest, "Board could not be parsed\n"))),
    }
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
