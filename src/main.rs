use std::io::{self, Write};
use rand::Rng;

const SIZE: usize = 3;

#[derive(Debug)]
enum GameState {
    Win,
    Lose,
    Draw,
    None,
}

impl GameState {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn render(field: [[char; SIZE]; SIZE]) {
    for row in field {
        for (i, ceil) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("{ceil}");
            } else {
                print!("{ceil} | ");
            }
        }
        println!();
    }
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
    let row = rand::thread_rng().gen_range(0..SIZE);
    let column = rand::thread_rng().gen_range(0..SIZE);

    match field[row][column] {
        'x' => (),
        'o' => (),
        _ => {
            field[row][column] = 'o';
            return;
        } 
    }

    for row in field {
        for (i, ceil) in row.iter().enumerate() {
            match ceil {
                'x' => (),
                'o' => (),
                _ => {
                    row[i] = 'o';
                    return;
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
        render(field);

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

    render(field);
    println!();
    state.show();
}
