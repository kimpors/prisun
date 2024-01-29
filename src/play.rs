pub mod bot;
pub mod player;

pub use self::player::Player;
pub use self::bot::Bot;

use super::Game;

pub trait Play {
    fn make_move(&self, game: &mut Game) -> Result<(), &'static str>; 
}
