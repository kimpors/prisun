use std::io::{self, Write};
use crate::{state::State, Game};

pub enum PlayerType {
    Player,
    Bot,
}

impl PlayerType {
    pub fn r#move(&self, game: &mut Game) -> Result<(), &'static str> {
        match self {
            PlayerType::Player => move_player(game),
            PlayerType::Bot => move_bot(game),
        }
    }
}

fn move_player(game: &mut Game) -> Result<(), &'static str> {
    let size = game.size as usize;

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

    if !(1..size * size).contains(&index) {
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

fn move_bot(game: &mut Game) -> Result<(), &'static str> {
    let size = game.size as usize;

    match game.set(1, 1) {
        Some(value) =>{ 
            if value.is_numeric() {
                *value = 'c';
            }

            return Ok(());
        },

        None => (),
    };

    let mut wins = Vec::new();

    for y in 0..size {
        for x in 0..size {
            if game.field[y][x].is_numeric() {
                let other = game.field[y][x];
                game.field[y][x] = 'o';

                if let State::Lose = game.check_win() {
                    game.field[y][x] = other;
                    wins.push((y, x));
                    continue;
                }

                game.field[y][x] = other;
            }
        }
    }

    if !wins.is_empty() {
        let w = wins[0];

        match game.set(w.0, w.1) {
            Some(value) => {
                *value = 'o';
            },
            None => return Err("Error, while changing ceil."),
        }
    }

    for row in &mut game.field {
        for col in row {
            if col.is_numeric() {
                *col = 'o';
                return Ok(());
            }
        }
    }

    return Err("Error, can't find any possible move.");
}
