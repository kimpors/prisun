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


    if index < 1 || index > size * size {
        return Err("Argument don't in range.");
    }

    let mut row = 0;
    let mut col = index;

    while col > 3 {
        col -= 3;
        row += 1;
    }

    if game.field[row][col - 1].is_numeric() {
        game.field[row][col - 1] = 'x';
    } else {
        return Err("Already filled ceil.");
    }

    return Ok(());
}

fn move_bot(game: &mut Game) -> Result<(), &'static str> {
    let size = game.size as usize;

    if game.field[1][1].is_numeric() {
        game.field[1][1] = 'o';
        return Ok(());
    }

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
        game.field[w.0][w.1] = 'o';
        return Ok(());
    }

    for y in 0..size {
        for x in 0..size {
            if game.field[y][x].is_numeric() {
                game.field[y][x] = 'o';
                return Ok(());
            }
        }
    }

    return Err("Your opponent can't find a possible move.");
}
