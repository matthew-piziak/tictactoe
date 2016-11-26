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
use std::fmt;

fn root_handler(req: &mut Request) -> IronResult<Response> {
    // TODO: to response
    let params = req.get_ref::<UrlEncodedQuery>().expect("Could not read query parameters");
    let board_param =
        params["board"].first().expect("Could not read `board` query parameter").clone();
    match Board::try_from(board_param) {
        Ok(board) => Ok(Response::with((status::Ok, format!("{}", board)))),
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

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &Marker::X => "x",
                   &Marker::O => "o",
                   &Marker::Empty => " ",
               })
    }
}

#[derive(Debug)]
struct Board {
    markers: [Marker; 9],
}

impl Board {
    fn has_triple(&self, marker: Marker) -> bool {
        // rows
        (self.markers[0] == marker && self.markers[1] == marker && self.markers[2] == marker) ||
        (self.markers[3] == marker && self.markers[4] == marker && self.markers[5] == marker) ||
        (self.markers[6] == marker && self.markers[7] == marker && self.markers[9] == marker) ||
        // columns
        (self.markers[0] == marker && self.markers[3] == marker && self.markers[6] == marker) ||
        (self.markers[1] == marker && self.markers[4] == marker && self.markers[7] == marker) ||
        (self.markers[2] == marker && self.markers[5] == marker && self.markers[8] == marker) ||
        // diagonals
        (self.markers[0] == marker && self.markers[4] == marker && self.markers[8] == marker) ||
        (self.markers[2] == marker && self.markers[4] == marker && self.markers[6] == marker)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_string: String = self.markers.into_iter().map(display_marker).collect();
        write!(f, "{}", board_string)
    }
}

fn display_marker(marker: &Marker) -> char {
    match marker {
        &Marker::X => 'x',
        &Marker::O => 'o',
        &Marker::Empty => ' ',
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

        let mut count: HashMap<Marker, u8> = HashMap::new();
        for marker in markers.iter() {
            *count.entry(*marker).or_insert(0) += 1;
        }
        if (count[&Marker::O] == count[&Marker::X]) && (count[&Marker::Empty] != 0) {
            Err(())
        } else {
            Ok(Board { markers: markers })
        }
    }
}
