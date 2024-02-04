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
    pub fn calculate(game: &mut Game) -> State {
        let win = vec!['x', 'x', 'x'];
        let lose = vec!['o', 'o', 'o'];
        let none = vec!['-', '/', '|', '\\'];

        let mut vert = false;
        let mut horz = false;

        let field = &game.field;
        let len = game.row_len();

        for i in 0..len {
            for j in 0..len {
                if none.contains(&field[i][j]) {
                    continue;
                }

                if !field[i][j].is_numeric() {
                    if j + 2 < len {
                        vert = true;
                        let pattern = vec![field[i][j], field[i][j + 1], field[i][j + 2]];

                        if pattern == win {
                            game.field[i][j] = '-';
                            game.field[i][j + 1] = '-';
                            game.field[i][j + 2] = '-';


                            return State::Win;
                        } else if pattern == lose {
                            game.field[i][j] = '-';
                            game.field[i][j + 1] = '-';
                            game.field[i][j + 2] = '-';


                            return State::Lose;
                        } 
                    } 

                    if i + 2 < len {
                        horz = true;
                        let pattern = vec![field[i][j], field[i + 1][j], field[i + 2][j]];

                        if pattern == win {
                            game.field[i][j] = '|';
                            game.field[i + 1][j] = '|';
                            game.field[i + 2][j] = '|';


                            return State::Win;
                        } else if pattern == lose {
                            game.field[i][j] = '|';
                            game.field[i + 1][j] = '|';
                            game.field[i + 2][j] = '|';


                            return State::Lose;
                        } 
                    }

                    if vert && horz {
                        let pattern = vec![field[i][j], field[i + 1][j + 1], field[i + 2][j + 2]];

                        if pattern == win {
                            game.field[i][j] = '\\';
                            game.field[i + 1][j + 1] = '\\';
                            game.field[i + 2][j + 2] = '\\';
                            

                            return State::Win;
                        } else if pattern == lose {
                            game.field[i][j] = '\\';
                            game.field[i + 1][j + 1] = '\\';
                            game.field[i + 2][j + 2] = '\\';


                            return State::Lose;
                        } 
                    }

                    if horz && j >= 2 {
                        let pattern = vec![field[i][j], field[i + 1][j - 1], field[i + 2][j - 2]];

                        if pattern == win {
                            game.field[i][j] = '/';
                            game.field[i + 1][j - 1] = '/';
                            game.field[i + 2][j - 2] = '/';


                            return State::Win;
                        } else if pattern == lose {
                            game.field[i][j] = '/';
                            game.field[i + 1][j - 1] = '/';
                            game.field[i + 2][j - 2] = '/';


                            return State::Lose;
                        } 
                    }

                    vert = false; 
                    horz = false;
                }
            }
        }

        return if is_draw(game) {
            State::Draw
        } else {
            State::None
        };
    }
}

fn is_draw(game: &Game) -> bool
{
    for row in &game.field {
        for ceil in row {
            if ceil.is_numeric() {
                return false;
            }
        }
    }

    return true;
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
