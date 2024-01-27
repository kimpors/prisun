use std::fmt;

use crate::Game;

#[derive(PartialEq)]
pub enum State {
    Win,
    Lose,
    Draw,
    None,
}

impl State {
    pub fn calculate(game: &mut Game) -> State {
        let win = vec!['x', 'x', 'x'];
        let lose = vec!['o', 'o', 'o'];

        // Vertical Pattern
        for row in &game.field {
            if *row == win {
                return State::Win;
            } else if *row == lose {
                return State::Lose;
            }
        }

        // Horizontal Pattern
        for i in 0..game.size as usize {
            let horizontal: &[char] = &[game.field[0][i], game.field[1][i], game.field[2][i]];

            if horizontal == win {
                return State::Win;
            } else if horizontal == lose {
                return State::Lose;
            }
        }

        // Diagonal Pattern
        for i in 0..2 {
            let diagonal: &[char] = &[game.field[0][i * 2], game.field[1][1], game.field[2][2 - i * 2]];

            if diagonal == win {
                return State::Win;
            } else if diagonal == lose {
                return State::Lose;
            }
        }

        // Check For A Draw
        for row in &game.field {
            for ceil in row {
                if ceil.is_numeric() {
                    return State::None;
                }
            }
        }

        return State::Draw;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Win => write!(f, "Win"),
            State::Lose => write!(f, "Lose"),
            State::Draw => write!(f, "Draw"),
            State::None => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_state() {
        let mut case1 = Game::new();

        assert!(State::calculate(&mut case1) == State::None);
    }

    #[test]
    fn display_output() {
        assert_eq!(State::Win.to_string(), "Win");
        assert_eq!(State::Lose.to_string(), "Lose");
        assert_eq!(State::Draw.to_string(), "Draw");
        assert_eq!(State::None.to_string(), "");
    }
}
