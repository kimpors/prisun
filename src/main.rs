use std::io::{self, Write};
use rand::Rng;

const SIZE: usize = 9;

#[derive(Debug)]
enum GameState {
    Win,
    Lose,
    None,
}

impl GameState {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn render(field: [char; SIZE]) {
    let length = SIZE / 3;

    for y in 0..length {
        for x in 0..length {
            if x == length - 1 {
                print!("{}", field[x + 3 * y]);
            } else {
                print!("{} | ", field[x + 3 * y]);
            }
        }
        println!();
    }
}

fn check_win(field: [char; SIZE]) -> GameState {
    let win = ['x', 'x', 'x'];
    let lose = ['o', 'o', 'o'];

    // Vertical
    if field[0..3] == win { return GameState::Win; }
    else if field[0..3] == lose { return GameState::Lose; }

    if field[3..6] == win { return GameState::Win; }
    else if field[3..6] == lose { return GameState::Lose; }

    if field[6..9] == win { return GameState::Win; }
    else if field[6..9] == lose { return GameState::Lose; }

    // Horizontal
    if [field[0], field[3], field[6]] == win { return GameState::Win; }
    else if [field[0], field[3], field[6]] == lose { return GameState::Lose; }

    if [field[1], field[4], field[7]] == win { return GameState::Win; }
    else if [field[1], field[4], field[7]] == lose { return GameState::Lose; }

    if [field[2], field[5], field[8]] == win { return GameState::Win; }
    else if [field[2], field[5], field[8]] == lose { return GameState::Lose; }

    // Diagonal
    if [field[0], field[4], field[8]] == win { return GameState::Win; }
    else if [field[0], field[4], field[8]] == lose { return GameState::Lose; }

    if [field[2], field[4], field[6]] == win { return GameState::Win; }
    else if [field[2], field[4], field[6]] == lose { return GameState::Lose; }

    return GameState::None;
}

fn bot_move(field: &mut [char; SIZE]) {
    let bot = rand::thread_rng().gen_range(0..SIZE);

    println!("{bot} -> {}", field[bot]);
    if field[bot] != 'x' && field[bot] != 'o' {
        field[bot] = 'o';
        return;
    }

    let mut i = 0;
    while i < SIZE {
        if field[i] != 'x' && field[i] != 'o' {
            field[i] = 'o';
            break;
        }

        i += 1;
    }
}

fn main() {
    let mut field = ['1', '2', '3',
                    '4', '5', '6',
                    '7', '8', '9'];

    let mut state = GameState::None;
    let mut count = 0;
    let mut result: usize;
    let mut buffer = String::new();

    println!("Tetris");
    while count <= SIZE / 2 {
        render(field);

        print!("\nEnter free ceil: ");
        io::stdout().flush();

        io::stdin()
            .read_line(&mut buffer)
            .expect("[Failed to read line]");

        result = buffer.trim().parse()
            .expect("[Enter a number]");

        buffer = String::new();

        if result < 1 || result > 9 { continue; }
        if field[result - 1] == 'x' || field[result - 1] == 'o' { continue; }

        field[result - 1] = 'x';
        bot_move(&mut field);

        match check_win(field) {
            GameState::Win => {
                state = GameState::Win;
                break;
            } ,
            GameState::Lose => {
                state = GameState::Lose;
                break;
            },
            GameState::None => count += 1,
        }
    }

    if let GameState::None = state {
        println!("Draw");
    } else {
        state.show();
    }

    render(field);
}
