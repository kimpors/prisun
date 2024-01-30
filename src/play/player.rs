use std::io::{self, Write};

use super::Play;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}

impl Play for Player {
    fn make_move(&self, game: &mut crate::Game) -> Result<(), &'static str> {
        let size = game.size;
        let mut buf = String::new();

        print!("\nEnter free ceil: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut buf)
            .unwrap();

        let index: usize = match buf.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err("Enter valid input."),
        };

        if !(1..=size * size).contains(&index) {
            return Err("Argument don't in range.");
        }

        let mut row = 0;
        let mut col = index;

        while col > 3 {
            col -= 3;
            row += 1;
        }

        match game.set(row, col - 1) {
            Some(value) => {
                if value.is_numeric() {
                    *value = 'x';
                }

                return Ok(());
            },
            None => return Err("Error while changing ceil."),
        }
    }
}
