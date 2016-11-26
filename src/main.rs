#![feature(try_from)]

extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use iron::status;
use urlencoded::UrlEncodedQuery;

use std::convert::TryFrom;
use std::env;
use std::collections::HashMap;

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let params = req.get_ref::<UrlEncodedQuery>().expect("Could not read query parameters");
    let board_param =
        params["board"].first().expect("Could not read `board` query parameter").clone();
    match Board::try_from(board_param) {
        Ok(board) => if board.is_o_turn() {
            Ok(Response::with((status::Ok, format!("Board: {:?}!", board))))
        } else {
            Ok(Response::with((status::BadRequest, "It is not O's turn")))
        },
        Err(()) => Ok(Response::with((status::BadRequest, "Board could not be parsed")))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Marker {
    X,
    O,
    Empty,
}

impl TryFrom<char> for Marker {
    type Err = ();

    fn try_from(c: char) -> Result<Marker, ()> {
        match c {
            'x' => Ok(Marker::X),
            'o' => Ok(Marker::O),
            ' ' => Ok(Marker::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Board {
    markers: [Marker; 9],
}

impl Board {
    fn is_o_turn(&self) -> bool {
        let mut count: HashMap<Marker, u8> = HashMap::new();
        for marker in self.markers.iter() {
            *count.entry(*marker).or_insert(0) += 1;
        }
        (count[&Marker::O] == count[&Marker::X]) && (count[&Marker::Empty] != 0)
    }
}

impl TryFrom<String> for Board {
    type Err = ();

    fn try_from(string: String) -> Result<Board, ()> {
        if string.len() != 9 {
            return Err(());
        }
        let mut markers: [Marker; 9] = [Marker::Empty; 9];
        for (i, c) in string.chars().enumerate() {
            match Marker::try_from(c) {
                Ok(marker) => markers[i] = marker,
                Err(_) => return Err(()),
            }
        }
        Ok(Board { markers: markers })
    }
}
