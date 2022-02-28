use crate::Word;
use rand::Rng;

const DICTIONARY: &str = include_str!("dictionary.txt");
const DICTIONARY_LEN: usize = 2309;

pub fn choose_random_word() -> Word {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..DICTIONARY_LEN);
    let chosen = DICTIONARY.lines().nth(n).unwrap();
    chosen.parse().unwrap()
}

pub fn is_in_list(word: &Word) -> bool {
    let word_str = word.to_string();
    DICTIONARY.lines().any(|l| l == word_str)
}
