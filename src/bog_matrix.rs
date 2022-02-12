mod matrix;
use matrix::*;

pub trait BogMatrix {
    fn contains(&self, check_word: &str) -> bool;
    fn print(&self);
}

fn find_word_recursive(matrix: &MatrixStruct<char>, word: &str, current: (usize, usize)) -> bool {
    if word.len() == 0 {
        return true;
    }
    if *matrix.at(current) != word.chars().next().unwrap() {
        return false;
    }
    let mut result: bool = false;
    if current.0 > 0 {
        result = result || find_word_recursive(matrix, &word[1..], (current.0 - 1, current.1));
        if current.1 > 0 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 - 1, current.1 - 1))
        }
        if current.1 < matrix.num_columns - 1 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 - 1, current.1 + 1))
        }
    }

    if current.0 < matrix.num_rows - 1 {
        result = result || find_word_recursive(matrix, &word[1..], (current.0 + 1, current.1));
        if current.1 > 0 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 + 1, current.1 - 1))
        }
        if current.1 < matrix.num_columns - 1 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 + 1, current.1 + 1))
        }
    }

    if current.1 > 0 {
        result = result || find_word_recursive(matrix, &word[1..], (current.0, current.1 - 1));
        if current.0 > 0 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 - 1, current.1 - 1));
        }
        if current.0 < matrix.num_rows - 1 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 + 1, current.1 - 1));
        }
    }

    if current.1 < matrix.num_columns - 1 {
        result = result || find_word_recursive(matrix, &word[1..], (current.0, current.1 + 1));
        if current.0 > 0 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 - 1, current.1 + 1));
        }
        if current.0 < matrix.num_rows - 1 {
            result =
                result || find_word_recursive(matrix, &word[1..], (current.0 + 1, current.1 + 1));
        }
    }
    result
}

impl BogMatrix for MatrixStruct<char> {
    fn contains(&self, word: &str) -> bool {
        for i in 0..self.num_columns {
            for j in 0..self.num_rows {
                if find_word_recursive(self, word, (i, j)) {
                    return true;
                }
            }
        }
        false
    }

    fn print(&self) {
        self.print();
    }
}

pub fn create(num_rows: usize, num_columns: usize) -> impl BogMatrix {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut mat = MatrixStruct::<char> {
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

#[cfg(test)]
mod tests {
    use crate::bog_matrix::matrix::MatrixStruct;
    use crate::bog_matrix::BogMatrix;
    pub fn dummy_matrix_create(num_rows: usize, num_columns: usize) -> impl BogMatrix {
        let mut mat = MatrixStruct::<char> {
            matrix: Vec::with_capacity(num_rows * num_columns),
            num_rows: num_rows,
            num_columns: num_columns,
        };
        for _ in (0..num_rows * num_columns).step_by(5) {
            mat.matrix.push('A');
            mat.matrix.push('B');
            mat.matrix.push('C');
            mat.matrix.push('D');
            mat.matrix.push('E');
        }
        mat
    }

    #[test]
    fn matrix_contains_test() {
        let matrix: Box<dyn BogMatrix> = Box::new(dummy_matrix_create(5, 5));
        assert_eq!(matrix.contains("ABBAABCDED"), true);
        assert_eq!(matrix.contains("GGGG"), false);
        assert_eq!(matrix.contains("ABBEABCDED"), false);
    }
}
