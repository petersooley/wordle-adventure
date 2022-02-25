use rand::Rng;
use crate::Word;

const DICTIONARY: &str = include_str!("dictionary.txt");
const DICTIONARY_LEN: usize = 2309;

pub fn choose_random_word() -> Word {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..DICTIONARY_LEN);
    let chosen = DICTIONARY.lines().nth(n).unwrap();
    chosen.parse().unwrap()
}
