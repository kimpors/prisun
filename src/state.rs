use crate::field::Field;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum State {
    Win,
    Lose,
    Draw,
    None,
}

impl State {
    pub fn predict(field: &mut Field, enemy: char) -> State {
        if let State::Win = State::check(field, enemy, false) {
            State::Lose
        } else {
            State::None
        }
    }


    pub fn check(field: &mut Field, skin: char, fill_pattern: bool) -> State {
        let size = field.get_size();

        for i in 0..size {
            for j in 0..size - 2 {
                if field[i][j] != skin {
                    continue;
                }

                if field[i][j + 1] == skin && field[i][j + 2] == skin {
                    if fill_pattern {
                        field[i][j] = '-';
                        field[i][j + 1] = '-';
                        field[i][j + 2] = '-';
                    }

                    return State::Win;
                } 
            }
        }

        for i in 0..size - 2 {
            for j in 0..size{
                if field[i][j] != skin {
                    continue;
                }

                if field[i + 1][j] == skin && field[i + 2][j] == skin {
                    if fill_pattern {
                        field[i][j] = '|';
                        field[i + 1][j] = '|';
                        field[i + 2][j] = '|';
                    }

                    return State::Win;
                } 
            }
        }

        for i in 0..size - 2 {
            for j in 0..size - 2 {
                if field[i][j] != skin {
                    continue;
                }

                if field[i + 1][j + 1] == skin && field[i + 2][j + 2] == skin {
                    if fill_pattern {
                        field[i][j] = '\\';
                        field[i + 1][j + 1] = '\\';
                        field[i + 2][j + 2] = '\\';
                    }

                    return State::Win;
                }
            }
        }

        for i in 2..size {
            for j in 0..size - 2 {
                if field[i][j] != skin {
                    continue;
                }

                if field[i - 1][j + 1] == skin && field[i - 2][j + 2] == skin {
                    if fill_pattern {
                        field[i][j] = '/';
                        field[i - 1][j + 1] = '/';
                        field[i - 2][j + 2] = '/';
                    }

                    return State::Win;
                }
            }
        }

        for i in 0..size {
            for j in 0..size {
                if field[i][j].is_numeric() {
                    return State::None;
                }
            }
        }

        State::Draw
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn output() {
        assert_eq!(State::Win.to_string(), "Win");
        assert_eq!(State::Lose.to_string(), "Lose");
        assert_eq!(State::Draw.to_string(), "Draw");
        assert_eq!(State::None.to_string(), "");
    }

    #[test]
    fn check_logic() {
        let mut cases = logic_cases();

        assert_eq!(State::check(&mut cases[0], 'x', false), State::Win);
        assert_eq!(State::check(&mut cases[1], 'x', false), State::Win);
        assert_eq!(State::check(&mut cases[2], 'x', false), State::Win);
        assert_eq!(State::check(&mut cases[3], 'x', false), State::Win);
    }

    #[test]
    fn predict_logic() {
        let mut cases = logic_cases();

        assert_eq!(State::predict(&mut cases[0], 'x'), State::Lose);
        assert_eq!(State::predict(&mut cases[1], 'x'), State::Lose);
        assert_eq!(State::predict(&mut cases[2], 'x'), State::Lose);
        assert_eq!(State::predict(&mut cases[3], 'x'), State::Lose);
    }


    fn logic_cases() -> [Field; 4] {
        let mut vert = Field::with_size(4);
        let mut horz = Field::with_size(4);
        let mut diag_left = Field::with_size(4);
        let mut diag_right = Field::with_size(4);

        vert[1][1] = 'x';
        vert[1][2] = 'x';
        vert[1][3] = 'x';

        horz[1][1] = 'x';
        horz[1][2] = 'x';
        horz[1][3] = 'x';

        diag_left[1][1] = 'x';
        diag_left[2][2] = 'x';
        diag_left[3][3] = 'x';

        diag_right[1][3] = 'x';
        diag_right[2][2] = 'x';
        diag_right[3][1] = 'x';

        [vert, horz, diag_left, diag_right]
    } 
}
