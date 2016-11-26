//! The Tic-tac-toe board.

use std::convert::TryFrom;
use std::fmt;
use std::collections::HashMap;

/// Plays the board by one move from O's perspective. O is the first player.
pub fn play<'a>(board: Board) -> Board {
    let mut children: Vec<Board> = vec![];
    let mut minimaxen: Vec<GameResult> = vec![];
    for child in board.children(&Player::O) {
        let game_result = child.minimax(&Player::X);
        children.push(child);
        minimaxen.push(game_result);
    }
    match minimaxen.iter().position(|&ref game_result| *game_result == GameResult::OWins) {
        Some(position) => {
            let winner = children.get(position).unwrap();
            return winner.clone();
        }
        None => {}
    };
    match minimaxen.iter().position(|&ref game_result| *game_result == GameResult::Draw) {
        Some(position) => {
            let drawer = children.get(position).unwrap();
            return drawer.clone();
        }
        None => {}
    };
    let loser = children.get(0).unwrap();
    return loser.clone();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Marker {
    X,
    O,
    Empty,
}

#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

impl From<Player> for Marker {
    fn from(player: Player) -> Marker {
        match player {
            Player::X => Marker::X,
            Player::O => Marker::O,
        }
    }
}

impl TryFrom<char> for Marker {
    type Err = ();

    fn try_from(c: char) -> Result<Marker, ()> {
        match c {
            'x' => Ok(Marker::X),
            'o' => Ok(Marker::O),
            '+' => Ok(Marker::Empty),
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

/// The Tic-tac-toe board.
///
/// Markers are read across the board left-to-right, top-to-bottom:
/// 0 1 2
/// 3 4 5
/// 6 7 8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    markers: [Marker; 9],
}

impl Board {
    fn has_triple(&self, player: Player) -> bool {
        let marker: Marker = Marker::from(player);
        // rows
        (self.markers[0] == marker && self.markers[1] == marker && self.markers[2] == marker) ||
        (self.markers[3] == marker && self.markers[4] == marker && self.markers[5] == marker) ||
        (self.markers[6] == marker && self.markers[7] == marker && self.markers[8] == marker) ||
        // columns
        (self.markers[0] == marker && self.markers[3] == marker && self.markers[6] == marker) ||
        (self.markers[1] == marker && self.markers[4] == marker && self.markers[7] == marker) ||
        (self.markers[2] == marker && self.markers[5] == marker && self.markers[8] == marker) ||
        // diagonals
        (self.markers[0] == marker && self.markers[4] == marker && self.markers[8] == marker) ||
        (self.markers[2] == marker && self.markers[4] == marker && self.markers[6] == marker)
    }

    fn children(&self, next_player: &Player) -> Vec<Board> {
        let mut children: Vec<Board> = vec![];
        for (index, marker) in self.markers.iter().enumerate() {
            if *marker == Marker::Empty {
                let mut child_markers = self.markers.clone();
                child_markers[index] = Marker::from(next_player.clone());
                children.push(Board { markers: child_markers });
            }
        }
        children
    }

    fn minimax(&self, next_player: &Player) -> GameResult {
        if self.has_triple(Player::X) {
            return GameResult::XWins;
        } else if self.has_triple(Player::O) {
            return GameResult::OWins;
        } else {
            let mut minimaxen: Vec<GameResult> = vec![];
            let next_next_player = match *next_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            for child in self.children(next_player) {
                minimaxen.push(child.minimax(&next_next_player))
            }
            match *next_player {
                Player::O => {
                    if minimaxen.contains(&GameResult::OWins) {
                        return GameResult::OWins;
                    } else if minimaxen.contains(&GameResult::Draw) {
                        return GameResult::Draw;
                    } else {
                        return GameResult::XWins;
                    }
                }
                Player::X => {
                    if minimaxen.contains(&GameResult::XWins) {
                        return GameResult::XWins;
                    } else if minimaxen.contains(&GameResult::Draw) {
                        return GameResult::Draw;
                    } else {
                        return GameResult::OWins;
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    XWins,
    OWins,
    Draw,
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
                Err(_) => {
                    println!("Failing marker: {:?}", c);
                    return Err(());
                }
            }
        }

        let mut count: HashMap<Marker, u8> = HashMap::new();
        for marker in markers.iter() {
            *count.entry(*marker).or_insert(0) += 1;
        }
        if count[&Marker::Empty] == 9 {
            Ok(Board { markers: markers })
        } else if (count[&Marker::O] != count[&Marker::X]) || (count[&Marker::Empty] == 0) {
            println!("Failing markers: {:?}", markers);
            Err(())
        } else {
            Ok(Board { markers: markers })
        }
    }
}

#[test]
fn parse_example() {
    use self::Marker::*;
    let board = Board::try_from("+xxo++o++".into());
    assert_eq!(board,
               Ok(Board { markers: [Empty, X, X, O, Empty, Empty, O, Empty, Empty] }));
}

#[test]
fn parse_empty_board() {
    use self::Marker::*;
    let board = Board::try_from("+++++++++".into());
    assert_eq!(board,
               Ok(Board {
                   markers: [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
               }));
}

#[test]
fn o_wins_example() {
    use self::Marker::*;
    let board = Board::try_from("+xxo++o++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(next_board,
               Board { markers: [O, X, X, O, Empty, Empty, O, Empty, Empty] });
    assert!(next_board.has_triple(Player::O));
}

#[test]
fn optimal_first_move() {
    use self::Marker::*;
    let board = Board::try_from("+++++++++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(next_board,
               Board { markers: [O, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty] });
}


// There are multiple perfect games. This is just verifying that the behavior
// doesn't change as I refactor.
#[test]
fn perfect_game() {
    let board = Board::try_from("+++++++++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(format!("{}", next_board), "o        ");
    let board = Board::try_from("o+++x++++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(format!("{}", next_board), "oo  x    ");
    let board = Board::try_from("oox+x++++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(format!("{}", next_board), "oox x o  ");
    let board = Board::try_from("ooxxx+o++".into()).unwrap();
    let next_board = play(board);
    assert_eq!(format!("{}", next_board), "ooxxxoo  ");
    let board = Board::try_from("ooxxx+oox".into()).unwrap();
    let next_board = play(board);
    assert_eq!(format!("{}", next_board), "ooxxxooox");
}
