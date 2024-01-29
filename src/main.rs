use prisun::Game;
use prisun::play::{Bot, Play, Player};
use prisun::state::State;

fn main() {
    let mut game = Game::new();

    let hero = Player::new();
    let enemy = Bot::new();

    loop {
        game.draw();

        match hero.make_move(&mut game) {
            Ok(_) => (),
            Err(_) => continue,
        }

        match game.check_win() {
            State::None => (),
            _ => break,
        }

        match enemy.make_move(&mut game) {
            Ok(_) => (),
            Err(_) => continue,
        }

        match game.check_win() {
            State::None => (),
            _ => break,
        }
    };

    game.draw();
}
