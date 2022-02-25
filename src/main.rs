mod dictionary;

fn main() {
    let answer = dictionary::choose_random_word();
    println!("today's answer: {}", answer);
}
