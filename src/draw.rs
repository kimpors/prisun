pub mod text;

use crate::{state::State, Game};
pub use self::text::Text;

pub trait Draw {
    fn draw(&self, game: &Game, state: &State);
}
