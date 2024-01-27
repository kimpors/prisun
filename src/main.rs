use prisun::Game;
use prisun::player::Type;
use prisun::state::State;

fn main() {
    let mut game = Game::new();

    let hero = Type::Player;
    let enemy = Type::Bot;

    loop {
        game.draw();

        match hero.r#move(&mut game) {
            Ok(_) => (), 
            Err(_) => continue,
        };

        match game.check_win() {
            State::None => (),
            _ => break,
        }

        match enemy.r#move(&mut game) {
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
