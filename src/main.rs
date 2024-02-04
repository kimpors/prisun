use prisun::draw::{Draw, Text};
use prisun::Game;
use prisun::play::{Bot, Play, Player};
use prisun::state::State;

fn main() {
    let mut game = Game::new();
    let render = Text::new();

    let hero = Player::new();
    let enemy = Bot::new();


    let state = loop {
        render.draw(&game, &State::None);

        match hero.make_move(&mut game) {
            Ok(_) => (),
            Err(_) => continue,
        }

        match State::calculate(&game) {
            State::None => (),
            other => {
                if game.len() < 100 {
                    game.expand()
                } else {
                    break other;
                }
            },
        }

        match enemy.make_move(&mut game) {
            Ok(_) => (),
            Err(_) => continue,
        }

        match State::calculate(&game) {
            State::None => (),
            other => {
                if game.len() < 100 {
                    game.expand()
                } else {
                    break other;
                }
            },
        }
    };

    render.draw(&game, &state);
}
