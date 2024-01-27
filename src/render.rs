use termion::{color, terminal_size};

use crate::{state::State, Game};

pub enum Mode {
    Terminal,
}

impl Mode {
    pub fn draw(&self, game: &Game) {
        match self {
            Mode::Terminal => draw_terminal(game),
        }
    }
}


fn draw_terminal(game: &Game) {
    let offset = terminal_size().unwrap();
    let offset = (offset.0 / 16, offset.1 / 2);

    let pos = termion::cursor::Goto(1, offset.1);
    let clear = termion::clear::All;

    println!("{clear}{pos}");

    // Header
    match &game.state {
        State::None => (),
        other => {
            print!("{}", color::Fg(color::Yellow));
            println!("{}{}{}", "\t".repeat(offset.0 as usize),  " ".repeat(8 - other.to_string().len()), other);
            println!("{}", color::Fg(color::Reset));
        } 
    }

    print!("{}", color::Fg(color::Blue));


    // Main Field
    for row in &game.field {
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
