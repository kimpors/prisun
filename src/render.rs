use termion::{color, terminal_size};

use crate::state::State;

pub enum Mode {
    Terminal,
}

impl Mode {
    pub fn draw(&self, state: &State, field: &Vec<Vec<char>>) {
        match self {
            Mode::Terminal => draw_terminal(state, field),
        }
    }
}


fn draw_terminal(state: &State, field: &Vec<Vec<char>>) {
    let offset = offset_terminal();

    let pos = termion::cursor::Goto(1, offset.1);
    let clear = termion::clear::All;

        println!("{clear}{pos}");

        // Header
        match state {
            State::None => (),
            other => {
                print!("{}", color::Fg(color::Yellow));
                println!("{}{}{}", "\t".repeat(offset.0 as usize),  " ".repeat(8 - other.to_string().len()), other);
                println!("{}", color::Fg(color::Reset));
            } 
        }

        print!("{}", color::Fg(color::Blue));


        // Main Field
        for row in field {
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

fn offset_terminal() -> (u16, u16) {
    let size = terminal_size().unwrap();
    return (size.0 / 16, size.1 / 2);
}
