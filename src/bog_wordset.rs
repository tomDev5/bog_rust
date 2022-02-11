use std::collections::HashSet;

struct BogWordsetStruct {
    words: HashSet<String>,
}

pub trait BogWordSet {
    fn add_word(&mut self, new_word: String);
    fn contains(&self, check_word: &String) -> bool;
    fn remove(&mut self, to_remove: &String) -> bool;
    fn print(&self);
}

impl BogWordSet for BogWordsetStruct {
    fn add_word(&mut self, new_word: String) {
        self.words.insert(new_word);
    }

    fn contains(&self, check_word: &String) -> bool {
        self.words.contains(check_word)
    }

    fn remove(&mut self, to_remove: &String) -> bool {
        self.words.remove(to_remove)
    }

    fn print(&self) {
        print!("Printing wordset:\t");
        // for word in &self.words {
        //     println!("\t{}", word);
        // }
        println!("{:?}", self.words);
    }
}

pub fn bog_wordset_create() -> impl BogWordSet {
    let set = BogWordsetStruct {
        words: HashSet::new(),
    };
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_set_add_contains_test() {
        let mut word_set = bog_wordset_create();
        word_set.add_word(String::from("Word"));
        word_set.add_word(String::from("Another"));
        word_set.add_word(String::from("Third"));
        assert_eq!(word_set.contains(&String::from("Word")), true);
        assert_eq!(word_set.contains(&String::from("Another")), true);
        assert_eq!(word_set.contains(&String::from("Third")), true);
    }

    #[test]
    fn word_set_remove_test() {
        let mut word_set = bog_wordset_create();
        word_set.add_word(String::from("Word"));
        word_set.add_word(String::from("Another"));
        word_set.add_word(String::from("Third"));
        word_set.remove(&String::from("Another"));
        assert_eq!(word_set.contains(&String::from("Word")), true);
        assert_eq!(word_set.contains(&String::from("Another")), false);
        assert_eq!(word_set.contains(&String::from("Third")), true);
    }
}
