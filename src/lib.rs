pub mod state;
pub mod render;
pub mod player;

use render::RenderMode;
use state::State;

pub struct Game {
    size: u8,
    field: Vec<Vec<char>>,
    render: RenderMode,
    state: State,
}


impl Game {
    pub fn new() -> Self {
        Self {
            size: 3,
            state: State::None,
            render: RenderMode::Terminal,
            field: vec![vec!['1', '2', '3'],
                        vec!['4', '5', '6'],
                        vec!['7', '8', '9']],
        }
    }

    pub fn check_win(&mut self) -> &State {
        self.state = State::calculate(self);
        return &self.state;
    }

    pub fn draw(&self) {
        self.render.draw(&self.state, &self.field);
    }
}
