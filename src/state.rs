use std::fmt;

use crate::Game;

#[derive(PartialEq, Debug)]
pub enum State {
    Win,
    Lose,
    Draw,
    None,
}

impl State {
    pub fn calculate(game: &Game) -> State {
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
        let cases = cases();

        assert_eq!(cases[0].0, cases[0].1);
        assert_eq!(cases[1].0, cases[1].1);
        assert_eq!(cases[2].0, cases[2].1);
        assert_eq!(cases[3].0, cases[3].1);
    }

    #[test]
    fn display_output() {
        assert_eq!(State::Win.to_string(), "Win");
        assert_eq!(State::Lose.to_string(), "Lose");
        assert_eq!(State::Draw.to_string(), "Draw");
        assert_eq!(State::None.to_string(), "");
    }

    fn cases() -> Vec<(State, State)> {
        let case1 = Game::new();

        let case2 = Game::build_with()
            .field(vec![vec!['x', 'x', 'x'],
                        vec!['3', '4', '5'],
                        vec!['6', '7', '8']]).build();


        let case3 = Game::build_with()
            .field(vec![vec!['o', 'o', 'o'],
                        vec!['3', '4', '5'],
                        vec!['6', '7', '8']]).build();


        let case4 = Game::build_with()
            .field(vec![vec!['o', 'x', 'x'],
                        vec!['x', 'o', 'o'],
                        vec!['x', 'o', 'x']]).build();


        let case1 = (State::None, State::calculate(&case1));
        let case2 = (State::Win, State::calculate(&case2));
        let case3 = (State::Lose, State::calculate(&case3));
        let case4 = (State::Draw, State::calculate(&case4));

        return vec![case1, case2, case3, case4];
    }
}
