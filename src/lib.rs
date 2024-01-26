pub mod state;

use state::State;
use termion::{color, terminal_size};

pub struct Game {
    size: u8,
    field: Vec<Vec<char>>,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            size: 3,
            state: State::None,
            field: vec![vec!['1', '2', '3'],
                        vec!['4', '5', '6'],
                        vec!['7', '8', '9']],
        }
    }

    pub fn set(&mut self, index: u8) -> Result<(), &'static str> {
        let size = &self.size;
        let field = &mut self.field;

        if index < 1 || index > size * size {
            return Err("Argument don't in range.");
        }

        let mut row = 0;
        let mut col = index as usize;

        while col > 3 {
            col -= 3;
            row += 1;
        }

        if field[row][col - 1].is_numeric() {
            field[row][col - 1] = 'x';
        } else {
            return Err("Already filled ceil.");
        }

        return Ok(());
    }

    pub fn bot_move(&mut self) {
        let size = self.size as usize;

        if self.field[1][1].is_numeric() {
            self.field[1][1] = 'o';
            return;
        }


        let mut wins = Vec::new();

        for y in 0..size {
            for x in 0..size {
                if self.field[y][x].is_numeric() {
                    let other = self.field[y][x];
                    self.field[y][x] = 'o';

                    if let State::Lose = self.check_win() {
                        self.field[y][x] = other;
                        wins.push((y, x));
                        continue;
                    }

                    self.field[y][x] = other;
                }
            }
        }

        if !wins.is_empty() {

            let w = wins[0];
            self.field[w.0][w.1] = 'o';
            return;
        }

        for row in &mut self.field {
            for col in row {
                if col.is_numeric() {
                    *col = 'o';
                    return;
                }
            }
        }
    }


    pub fn check_win(&mut self) -> &State {
        self.state = State::calculate(self);
        return &self.state;
    }


    pub fn draw(&self) {
        let offset = self.get_offset();

        let pos = termion::cursor::Goto(1, offset.1);
        let clear = termion::clear::All;

        println!("{clear}{pos}");

        // Header
        match &self.state {
            State::None => (),
            other => {
                print!("{}", color::Fg(color::Yellow));
                println!("{}{}{}", "\t".repeat(offset.0 as usize),  " ".repeat(8 - other.to_string().len()), other);
                println!("{}", color::Fg(color::Reset));
            } 
        }

        print!("{}", color::Fg(color::Blue));


        // Main Field
        for row in &self.field {
            print!("{}", "\t".repeat(offset.0 as usize));

            for (i, ceil) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    print!(" {ceil} ");
                } else {
                    print!(" {ceil} | ");
                }
            }

            println!("\n{}{}", "\t".repeat(offset.0.into()), "-".repeat(13));
        }

        print!("{}", color::Fg(color::Red));
    }

    fn get_offset(&self) -> (u16, u16) {
        let size = terminal_size().unwrap();

        return (size.0 / 16, size.1 / 2);
    }
}
