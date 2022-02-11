struct BogMatrixStruct<T: Copy> {
    matrix: Vec<T>,
    num_rows: usize,
    num_columns: usize,
}

impl<T: Copy> BogMatrixStruct<T> {
    fn at(&self, i: usize, j: usize) -> T {
        assert_eq!(self.num_rows < i, true);
        assert_eq!(self.num_columns < j, true);
        let result = self.matrix[(i * self.num_columns) + j];
        result
    }
}

pub trait BogMatrix {
    fn contains(&self, check_word: &String) -> bool;
    fn print(&self);
}

impl<T: Copy + std::fmt::Display> BogMatrix for BogMatrixStruct<T> {
    fn contains(&self, check_word: &String) -> bool {
        // insert word finding algorithm
        true
    }

    fn print(&self) {
        for i in 0..self.num_rows {
            let start = i * self.num_columns;
            let end = start + self.num_columns;
            for j in start..end {
                print!("{} ", self.matrix[j]);
            }
            println!("\n");
        }
    }
}

pub fn bog_matrix_create(num_rows: usize, num_columns: usize) -> impl BogMatrix {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut mat = BogMatrixStruct::<char> {
        matrix: Vec::with_capacity(num_rows * num_columns),
        num_rows: num_rows,
        num_columns: num_columns,
    };

    for _ in 0..num_rows * num_columns {
        let letter: char = rng.gen_range(b'A'..b'Z') as char;
        mat.matrix.push(letter);
    }
    mat
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn word_set_add_contains_test() {
//         let mut word_set = bog_wordset_create();
//         word_set.add_word(String::from("Word"));
//         word_set.add_word(String::from("Another"));
//         word_set.add_word(String::from("Third"));
//         assert_eq!(word_set.contains(&String::from("Word")), true);
//         assert_eq!(word_set.contains(&String::from("Another")), true);
//         assert_eq!(word_set.contains(&String::from("Third")), true);
//     }
// }
