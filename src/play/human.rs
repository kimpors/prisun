use std::io::{self, Write};

use crate::field::Field;

use super::Play;

pub struct Human;

impl Play for Human {
    fn play(&self, skin: char, field: &mut Field) -> Option<char> {
        let mut buf = String::new();

        print!("\nEnter free ceil: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut buf)
            .unwrap();

        let index: usize = buf.trim().parse().ok()?;

        if !(1..=field.get_area()).contains(&index) {
            return None;
        }

        let mut row = 0;
        let mut col = index;
        let size = field.get_size();

        while col > size {
            col -= size;
            row += 1;
        }

        field[row][col - 1] = skin;
        Some(skin)
    }
}
