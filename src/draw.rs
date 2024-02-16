pub mod text;

use crate::field::Field;
pub use self::text::Text;

pub trait Draw {
    fn draw(&self, field: &Field);
}
