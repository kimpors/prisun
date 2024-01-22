extern crate termion;

use termion::color; 
use std::{collections::HashMap, fmt, io::{self, Write}};
use termion::terminal_size;

const SIZE: usize = 3;

enum GameState {
    Win,
    Lose,
    Draw,
    None,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameState::Win => write!(f, "Win"),
            GameState::Lose => write!(f, "Lose"),
            GameState::Draw => write!(f, "Draw"),
            GameState::None => write!(f, ""),
        }
    }
}

fn center_offset(cols: &u16) -> String {
    let mut offset = String::new();

    for _ in 0..cols / 16 {
        offset.push('\t');
    }

    return offset;
}


fn render(field: [[char; SIZE]; SIZE], state: &GameState) {
    let offset = center_offset(&terminal_size()
                               .expect("Error while getting terminal size.").0);

    match state {
        GameState::None => (),
        s => {
            print!("{}", color::Fg(color::Yellow));
            println!("{offset}{}{}", " ".repeat(8 - s.to_string().len()), s);
            println!("{}", color::Fg(color::Reset));
        } 
    }

    print!("{}", color::Fg(color::Blue));

    for row in field {
        print!("{offset}");

        for (i, ceil) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!(" {ceil} ");
            } else {
                print!(" {ceil} | ");
            }
        }

        println!("\n{offset}{}", "-".repeat(13));
    }

    print!("{}", color::Fg(color::Red));
}

fn check_win(field: [[char; SIZE]; SIZE]) -> GameState {
    let win: [char; SIZE] = ['x', 'x', 'x'];
    let lose: [char; SIZE] = ['o', 'o', 'o'];

    // Vertical Pattern
    for &row in field.as_slice() {
        if row == win {
            return GameState::Win;
        } else if row == lose {
            return GameState::Lose;
        }
    }

    // Horizontal Pattern
    for i in 0..SIZE {
        let horizontal: &[char] = &[field[0][i], field[1][i], field[2][i]];

        if horizontal == win {
            return GameState::Win;
        } else if horizontal == lose {
            return GameState::Lose;
        }
    }

    // Diagonal Pattern
    for i in 0..2 {
        let diagonal: &[char] = &[field[0][i * 2], field[1][1], field[2][2 - i * 2]];
        if diagonal == win {
            return GameState::Win;
        } else if diagonal == lose {
            return GameState::Lose;
        }
    }

    // Check For A Draw
    for row in field {
        for ceil in row {
            if ceil.is_numeric() {
                return GameState::None;
            }
        }
    }

    return GameState::Draw;
}

fn bot_move(field: &mut [[char; SIZE]; SIZE]) {
    match field[1][1] {
        'x' => (),
        'o' => (),
        _ => {
            field[1][1] = 'o';
            return;
        }
    }

    let mut variants = HashMap::new();

    for row in field {
        for (i, ceil) in row.iter().enumerate() {
            match ceil {
                'x' => (),
                'o' => (),
                c => {
                    row[i] = 'o';
                    variants.insert(i, check_win(field));
                    row[i] = *c;
                },
            };
        }
    }
}

fn main() {
    let mut field = [['1', '2', '3'],
                    ['4', '5', '6'],
                    ['7', '8', '9']];


    let mut row: usize;
    let mut column: usize;
    let mut buffer = String::new();

    println!("Tetris");
    let state = loop {
        render(field, &GameState::None);

        print!("\nEnter free ceil: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Error while readling a line.");

        column = buffer.trim().parse()
            .expect("Error, enter a number.");

        buffer = String::new();

        if column < 1 || column > 9 { continue; }

        row = 0;
        while column > 3 {
            column -= 3;
            row += 1;
        }

        match field[row][column - 1] {
            'x' => continue,
            'o' => continue,
            _ => field[row][column - 1] = 'x',
        }

        bot_move(&mut field);
        match check_win(field) {
            GameState::None => (),
            value => break value,
        }
    };

    render(field, &state);
}
