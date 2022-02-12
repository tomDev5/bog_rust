mod bog_wordset;
use bog_wordset::BogWordSet;

mod bog_matrix;
use bog_matrix::BogMatrix;

fn main() {
    let mut word_set = bog_wordset::bog_wordset_create();
    word_set.add_word(String::from("Word"));
    word_set.add_word(String::from("Another"));
    word_set.add_word(String::from("Third"));
    word_set.print();
    word_set.remove(&String::from("Another"));
    word_set.print();
    println!(
        "set contains Word?\t{}",
        word_set.contains(&String::from("Word"))
    );
    println!(
        "set contains Another?\t{}",
        word_set.contains(&String::from("Another"))
    );

    let matrix = bog_matrix::bog_matrix_create(5, 5);
    matrix.print();
    matrix.contains(&String::from("AA"));
    println!(
        "matrix contains AA?\t{}",
        matrix.contains(&String::from("AA"))
    );
    println!(
        "matrix contains BB?\t{}",
        matrix.contains(&String::from("BB"))
    );
}
