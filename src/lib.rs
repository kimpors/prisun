pub mod state;
pub mod play;
pub mod draw;
pub mod field;

use draw::{Draw, Text};
use field::Field;
use play::{Bot, Human, Player};
use state::State;

pub struct Game<'a, 'b> {
    field: Field,
    players: Vec<Player<'a>>,
    render: &'b dyn Draw,
}

impl Game<'_, '_> {
    pub fn new() -> Self {
        Self {
            field: Field::new(),
            players: vec![
                Player::new(&Human, 'x'),
                Player::new(&Bot, 'o'),
            ],
            render: &Text,
        }
    }

    pub fn build(field: Field, render: &'static impl Draw, players: Vec<Player<'static>>) -> Self {
        Self {
            field,
            render,
            players,
        }
    }

    pub fn play(&mut self) {
        let field = &mut self.field;
        let mut i = 0;

        'start: loop {
            while i < self.players.len() {
                self.render.draw(&field);

                match self.players[i].play(field) {
                    Some(skin) => {
                        match State::check(field, skin, true) {
                            State::Win => self.players[i].add_score(),
                            State::Draw => {
                                if field.get_size() < 9 {
                                    field.expand();
                                    field.clear();
                                } else {
                                    break 'start;
                                }

                            },
                            _ => (),
                        }

                        i += 1;
                    },
                    _ => (),
                }
            }

            i = 0;
        }


        self.render.draw(&field);
    }
}
