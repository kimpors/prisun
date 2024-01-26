use std::io::{self, Write};
use prisun::Game;
use prisun::state::State;

fn main() {
    let mut buf;
    let mut game = Game::new();

    loop {
        buf = String::new();
        game.draw();

        print!("\nEnter free ceil: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut buf)
            .unwrap();

        let index = match buf.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };


        if let Err(_) = game.set(index) { continue; }

        game.bot_move();


        match game.check_win() {
            State::None => (),
            _ => break,
        }
    };

    game.draw();
}
