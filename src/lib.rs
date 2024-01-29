pub mod state;
pub mod play;
pub mod draw;

use std::fmt::Display;


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
}


impl Game {
    pub fn new() -> Self {
        Self {
            size: 3,
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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_output() {
        let cases = cases();

        assert_eq!(cases[0].0, cases[0].1);
        assert_eq!(cases[1].0, cases[1].1);
        assert_eq!(cases[2].0, cases[2].1);
        assert_eq!(cases[3].0, cases[3].1);
    }

    fn cases() -> Vec<(String, String)> {
        let case1 = Game::new();

        let case2 = Game::build_with()
            .field(vec![vec!['1', 'x', '3'],
                        vec!['4', 'x', '6'],
                        vec!['7', 'x', '9']]).build();

        let case3 = Game::build_with()
            .field(vec![vec!['1', '2', '3'],
                        vec!['o', 'o', 'o'],
                        vec!['7', '8', '9']]).build();

        let case4 = Game::build_with()
            .field(vec![vec!['x', '2', 'o'],
                        vec!['4', 'o', '6'],
                        vec!['o', '8', 'x']]).build();

        let underline = "-".repeat(13);

        let case1 = (case1.to_string(), format!(" 1 |  2 |  3 \n{underline}\n \
                                                 4 |  5 |  6 \n{underline}\n \
                                                 7 |  8 |  9 \n{underline}\n"));

        let case2 = (case2.to_string(), format!(" 1 |  x |  3 \n{underline}\n \
                                                 4 |  x |  6 \n{underline}\n \
                                                 7 |  x |  9 \n{underline}\n"));

        let case3 = (case3.to_string(), format!(" 1 |  2 |  3 \n{underline}\n \
                                                 o |  o |  o \n{underline}\n \
                                                 7 |  8 |  9 \n{underline}\n"));

        let case4 = (case4.to_string(), format!(" x |  2 |  o \n{underline}\n \
                                                 4 |  o |  6 \n{underline}\n \
                                                 o |  8 |  x \n{underline}\n"));

        return vec![case1, case2, case3, case4];
    }
}
