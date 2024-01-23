extern crate termion;

use termion::color; 
use std::{fmt, io::{self, Write}};
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
    let size = terminal_size().unwrap();
    let offset = center_offset(&size.0);

    println!("{}", size.1);

    let pos = termion::cursor::Goto(1, size.1 / 2);
    let clear = termion::clear::All;

    println!("{clear}{pos}");


    // Header
    match state {
        GameState::None => (),
        s => {
            print!("{}", color::Fg(color::Yellow));
            println!("{offset}{}{}", " ".repeat(8 - s.to_string().len()), s);
            println!("{}", color::Fg(color::Reset));
        } 
    }

    print!("{}", color::Fg(color::Blue));

    // Main Field
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

    let mut wins: Vec<(usize, usize)> = Vec::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            match field[y][x] {
                'x' => (),
                'y' => (),
                c => {
                    field[y][x] = 'o';

                    match check_win(*field) {
                        GameState::Lose => wins.push((y, x)),
                        _ => {
                            print!("{}-", field[y][x]);
                            field[y][x] = c;
                            println!("{}", field[y][x]);
                        } ,
                    }
                }
            }
        }
    }

    if !wins.is_empty() {
        let first = wins.first().unwrap();
        field[first.0][first.1] = 'o';
        return;
    }

    for row in field {
        for ceil in row {
            match ceil {
                'x' => (),
                'o' => (),
                c => {
                    print!("{c}");
                    *c = 'o';
                    return;
                }
            }
        }
    }
}

fn main() {
    let mut field = [['1', '2', '3'],
                    ['4', '5', '6'],
                    ['7', '8', '9']];


    let mut row: usize;
    let mut column: usize;
    let mut buf;

    let state = loop {
        buf = String::new();
        render(field, &GameState::None);

        print!("\nEnter free ceil: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut buf)
            .unwrap();

        print!("{buf}");

        column = match buf.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        match column {
            1..=9 => (),
            _ => continue,
        }

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