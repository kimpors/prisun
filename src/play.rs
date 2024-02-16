pub mod bot;
pub mod human;

pub use self::human::Human;
pub use self::bot::Bot;

use crate::field::Field;

pub trait Play {
    fn play(&self, skin: char, field: &mut Field) -> Option<char>; 
}

pub struct Player<'a> {
    score: u8,
    skin: char,
    r#type: &'a dyn Play,
}

impl<'a> Player<'a> {
    pub fn new(r#type: &'a impl Play, skin: char) -> Self {
        Self {
            score: 0,
            r#type,
            skin,
        }
    } 

    pub fn play(&self, field: &mut Field) -> Option<char> {
        self.r#type.play(self.skin, field) 
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> u8 {
        self.score
    }

    pub fn get_skin(&self) -> char {
        self.skin
    }
}
