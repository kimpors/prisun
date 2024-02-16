use crate::field::Field;
use crate::state::State;
use super::Play;

pub struct Bot;

impl Play for Bot {
    fn play(&self, skin: char, field: &mut Field) -> Option<char> {
        let size = field.get_size();
        let skins = field.get_skins();

        if field[size / 2][size / 2].is_numeric() {
            field[size / 2][size / 2] = skin;
            return Some(skin);
        }

        let mut index = (0, 0);

        'start: for i in 0..size {
            for j in 0..size {
                if field[i][j].is_numeric() {
                    field[i][j] = skin;

                    match State::check(field, skin, false) {
                        State::Win => {
                            field[i][j] = '0';
                            index = (i, j);
                            break 'start;
                        },
                        State::None => index = (i, j),
                        _ => (),
                    }

                    for enemy in skins.keys() {
                        if *enemy == skin {
                            continue;
                        }

                        field[i][j] = *enemy;

                        if let State::Lose = State::predict(field, *enemy) {
                            field[i][j] = '0';
                            index = (i, j);
                            break 'start;
                        }
                    }

                    field[i][j] = '0';
                }
            }
        }

        field[index.0][index.1] = skin;
        Some(skin)
    }
}


#[cfg(test)]
mod test {
    use crate::play::Player;
    use super::*;

    #[test]
    fn logic() {
        let mut cases = cases();
        let bot = Player::new(&Bot, 'o');

        bot.play(&mut cases[0].0);
        bot.play(&mut cases[1].0);
        bot.play(&mut cases[2].0);

        assert_eq!(cases[0].0, cases[0].1);
        assert_eq!(cases[1].0, cases[1].1);
        assert_eq!(cases[2].0, cases[2].1);
    }

    fn cases() -> Vec<(Field, Field)> {
        let center = Field::with_size(4);
        let mut center_res = Field::with_size(4);

        let mut win = Field::with_size(4);
        let mut win_res = Field::with_size(4);

        let mut defense = Field::with_size(4);
        let mut defense_res = Field::with_size(4);

        center_res[2][2] = 'o';

        win[2][1] = 'o';
        win[2][2] = 'o';

        win_res[2][0] = 'o';
        win_res[2][1] = 'o';
        win_res[2][2] = 'o';

        defense[2][1] = 'x';
        defense[2][2] = 'x';

        defense_res[2][0] = 'o';
        defense_res[2][1] = 'x';
        defense_res[2][2] = 'x';

        vec![(center, center_res), (win, win_res), (defense, defense_res)]
    }
}
