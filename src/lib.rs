pub mod state;
pub mod render;
pub mod player;

use std::fmt::Display;

use render::Mode;
use state::State;

pub struct Build {
    game: Game,
}

impl Build {
    pub fn field(mut self, field: Vec<Vec<char>>) -> Build {
        self.game.field = field;

        return self;
    }

    pub fn build(self) -> Game {
        return self.game;
    }
}

pub struct Game {
    size: u8,
    field: Vec<Vec<char>>,
    render: Mode,
    state: State,
}


impl Game {
    pub fn new() -> Self {
        Self {
            size: 3,
            state: State::None,
            render: Mode::Terminal,
            field: vec![vec!['1', '2', '3'],
                        vec!['4', '5', '6'],
                        vec!['7', '8', '9']],
        }
    }

    pub fn build_with() -> Build {
        return Build { game: Game::new() };
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&char> {
        let row = match self.field.get(y) {
            Some(value) => value,
            None => return None,
        };

        match row.get(x) {
            Some(value) => return Some(value),
            None => return None,
        }
    }

    pub fn set(&mut self, y: usize, x: usize) -> Option<&mut char> {
        let row = match self.field.get(y) {
            Some(value) => value,
            None => return None,
        };

        match row.get(x) {
            Some(_) => (),
            None => return None,
        }

        return Some(&mut self.field[y][x]);
    }

    pub fn check_win(&mut self) -> &State {
        self.state = State::calculate(self);
        return &self.state;
    }

    pub fn draw(&self) {
        self.render.draw(self);
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();

        for row in &self.field {
            for (i, col) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    result.push_str(&format!(" {col} "));
                } else {
                    result.push_str(&format!(" {col} | "));
                }
            }

            result.push_str(&format!("\n{}\n", "-".repeat(13)));
        }

        return write!(f, "{result}");
    }
}
