use termion::{color, terminal_size};
use crate::{state::State, Game};

use super::Draw;

pub struct Text {}

impl Text {
    pub fn new() -> Self {
        Self {}
    }
}

impl Draw for Text {
    fn draw(&self, game: &Game, state: &State) {
        let offset = terminal_size().unwrap();

        let size = match game.to_string().split('\n')
                    .max_by_key(|a| a.len()) {
            Some(value) => value.len(),
            None => 0,
        };


        let offset = ((offset.0 / 7 - size as u16 / 2) / 2, offset.1 / 3);


        let pos = termion::cursor::Goto(1, offset.1);
        let clear = termion::clear::All;

        println!("{clear}{pos}");

        // Header
        match state {
            State::None => (),
            other => {
                print!("{}", color::Fg(color::Yellow));
                println!("{}{}{}", "\t".repeat(offset.0 as usize),  "\t".repeat(8 - other.to_string().len()), other);
                println!("{}", color::Fg(color::Reset));
            } 
        }

        print!("{}", color::Fg(color::Blue));

        // Main Field
        let mut is_first = true;
        for row in game.to_string().lines() {
            print!("{}", "\t".repeat(offset.0 as usize));

            for col in row.split_whitespace() {
                if is_first {
                    match col {
                        "x" => print!("|{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Green), color::Fg(color::Blue)),
                        "o" => print!("|{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Red), color::Fg(color::Blue)),
                        "/" | "|" | "\\" | "-" => print!("|{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Black), color::Fg(color::Blue)),
                        _ => print!("|{}{col}  |", " ".repeat(3 - col.len())),
                    }

                    is_first = false;
                } else {
                    match col {
                        "x" => print!("{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Green), color::Fg(color::Blue)),
                        "o" => print!("{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Red), color::Fg(color::Blue)),
                        "/" | "|" | "\\" | "-" => print!("{}{}{col}{}  |", " ".repeat(3 - col.len()), color::Fg(color::Black), color::Fg(color::Blue)),
                        _ => print!("{}{col}  |", " ".repeat(3 - col.len())),
                    }
                }
            }

            is_first = true;
            println!("\n{}{}", "\t".repeat(offset.0 as usize), "-".repeat(game.row_len() * 7 - game.row_len() + 1));
        }

        print!("{}", color::Fg(color::Magenta));
    }
}
