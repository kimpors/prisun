use crate::state::State;
use crate::Game;
use super::Play;

pub struct Bot {}

impl Bot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Play for Bot {
    fn make_move(&self, game: &mut Game) -> Result<(), &'static str> {
        let size = game.size as usize;

        match game.set(size / 2, size / 2) {
            Some(value) => { 
                if value.is_numeric() {
                    *value = 'o';
                    return Ok(());
                }
            },

            None => (),
        };

        let mut wins = Vec::new();
        let mut nones = Vec::new();

        for y in 0..size {
            for x in 0..size {
                if game.field[y][x].is_numeric() {
                    let other = game.field[y][x];
                    game.field[y][x] = 'o';

                    match game.check_win() {
                        State::Lose => wins.push((y, x)),
                        State::None => nones.push((y, x)),
                        _ => (),
                    }

                    game.field[y][x] = other;
                }
            }
        }

        for state in vec![&wins, &nones] {
            if !state.is_empty() {
                match game.set(state[0].0, state[0].1) {
                    Some(value) => {
                        *value = 'o';
                        return Ok(());
                    },

                    None => return Err("Error, while changing ceil."),
                }


            }
        }

        return Err("Error, can't find any possible move.");
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bot_move() {
        let bot = Bot::new();
        let mut  cases = cases();

        let _ = bot.make_move(&mut cases[0].0);
        assert_eq!(cases[0].0.to_string(), cases[0].1);

        let _ = bot.make_move(&mut cases[1].0);
        assert_eq!(cases[1].0.to_string(), cases[1].1);

        let _ = bot.make_move(&mut cases[2].0);
        assert_eq!(cases[2].0.to_string(), cases[2].1);

        let _ = bot.make_move(&mut cases[3].0);
        assert_eq!(cases[3].0.to_string(), cases[3].1);
    }

    fn cases() -> Vec<(Game, String)> {
        let case1 = Game::new();

        let case2 = Game::build_with()
            .field(vec![vec!['1', 'x', '3'],
                        vec!['4', 'o', 'o'],
                        vec!['x', 'x', '9']]).build();

        let case3 = Game::build_with()
            .field(vec![vec!['o', 'x', 'x'],
                        vec!['4', 'x', 'o'],
                        vec!['7', '8', '9']]).build();

        let case4 = Game::build_with()
            .field(vec![vec!['x', 'x', 'o'],
                        vec!['4', 'o', '6'],
                        vec!['7', '8', 'x']]).build();

        let underline = "-".repeat(13);

        let case1 = (case1, format!(" 1 |  2 |  3 \n{underline}\n \
                                                 4 |  o |  6 \n{underline}\n \
                                                 7 |  8 |  9 \n{underline}\n"));

        let case2 = (case2, format!(" 1 |  x |  3 \n{underline}\n \
                                                 o |  o |  o \n{underline}\n \
                                                 x |  x |  9 \n{underline}\n"));

        let case3 = (case3, format!(" o |  x |  x \n{underline}\n \
                                                 o |  x |  o \n{underline}\n \
                                                 7 |  8 |  9 \n{underline}\n"));

        let case4 = (case4, format!(" x |  x |  o \n{underline}\n \
                                                 4 |  o |  6 \n{underline}\n \
                                                 o |  8 |  x \n{underline}\n"));

        return vec![case1, case2, case3, case4];
    }
}
