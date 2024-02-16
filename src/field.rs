use std::{collections::HashMap, ops::{Deref, Index, IndexMut}};

#[derive(Debug, PartialEq)]
pub struct Field {
    size: usize,
    field: Vec<Vec<char>>,
}

impl Field {
    pub fn new() -> Self {
        let size = 3;

        Self {
            size,
            field: vec![
                vec!['0'; size],
                vec!['0'; size],
                vec!['0'; size]]
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            size,
            field: vec![vec!['0'; size]; size],
        }
    }

    pub fn get_size(&self) -> usize { self.size }
    pub fn get_area(&self) -> usize { self.size * self.size }

    pub fn get_skins(&self) -> HashMap<char, ()> {
        let size = self.size;
        let mut total = HashMap::with_capacity(6);

        for i in 0..size {
            for j in 0..size {
                match self.field[i][j] {
                    '|' | '\\' | '/' | '-' => (),
                    ch if ch.is_numeric() => (),
                    ch => {
                        total.entry(ch).or_insert(());
                    },
                }
            }
        }

        total
    }

    pub fn expand(&mut self) {
        self.size += 1;

        for row in &mut self.field {
            row.push('0');
        }

        self.field.push(vec!['0'; self.size]);
    }

    pub fn clear(&mut self) {
        let size = self.size;

        for i in 0..size {
            for j in 0..size {
                self.field[i][j] = '0';
            }
        }
    }
}

impl IndexMut<usize> for Field {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.field[index]
    }
}

impl Index<usize> for Field {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field[index]
    }
}

impl Deref for Field {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_size() {
        let temp = Field::with_size(4);

        assert_eq!(temp[0].len(), 4);
    }

    #[test]
    fn get_size() {
        let temp = Field::with_size(4);

        assert_eq!(temp.get_size(), 4);
    }

    #[test]
    fn get_area() {
        let temp = Field::with_size(4);

        assert_eq!(temp.get_area(), 16);
    }

    #[test]
    fn get_skins() {
        let mut temp = Field::new(); 
        
        temp[0][0] = 'x';
        temp[0][1] = 'z';
        temp[0][2] = 'o';

        let a = temp.get_skins();
        let mut b = HashMap::with_capacity(3);

        b.insert('x', ());
        b.insert('z', ());
        b.insert('o', ());

        assert_eq!(a, b);
    }

    #[test]
    fn expand() {
        let mut temp = Field::new();
        temp.expand();

        assert_eq!(temp[0].len(), 4);
    }

    #[test]
    fn clear() {
        let mut a = Field::new();
        let b = Field::new();

        a[0][0] = 'x';
        a[0][1] = 'z';
        a[0][2] = 'o';

        a.clear();

        assert_eq!(a, b);
    }
}
