use termion::{color, terminal_size};
use crate::field::Field;

use super::Draw;

pub struct Text;

impl Draw for Text {
    fn draw(&self, field: &Field) {
        let offset = terminal_size().unwrap();
        let size = field.get_size();

        let offset = ((offset.0 / 7 - size as u16 / 2) / 2, offset.1 / 3);

        print!("{}{}", 
               termion::cursor::Goto(1, offset.1), 
               termion::clear::All);


        let red = color::Fg(color::Red);
        let blue = color::Fg(color::Blue);
        let black = color::Fg(color::Black);
        let magenta = color::Fg(color::Magenta);


        print!("{blue}");
        let margin = "\t".repeat(offset.0 as usize);
        let line = "-".repeat(field.get_size() * 7 - field.get_size() + 1);

        let mut count = 1;

        for row in field.iter() {
            print!("{margin}|");

            for col in row {
                let (left, right) = if count > 99 {
                    (" ", " ")
                } else if count > 9 {
                    ("  ", " ")
                } else {
                    ("  ", "  ")
                };

                match col {
                    '/' | '|' | '\\' | '-' => print!("  {black}{col}{blue}  "),
                    _ if col.is_numeric() => print!("{left}{count}{right}"),
                    _ => print!("  {red}{col}{blue}  "),
                }

                print!("|");
                count += 1;
            }

            println!("\n{margin}{line}");
        }

        print!("{magenta}");
    }
}
