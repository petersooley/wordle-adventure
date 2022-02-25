use rand::Rng;

const DICTIONARY: &str = include_str!("dictionary.txt");
const DICTIONARY_LEN: usize = 2309;

pub fn choose_random_word() -> &'static str {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..DICTIONARY_LEN);
    DICTIONARY.lines().nth(n).unwrap()
}
