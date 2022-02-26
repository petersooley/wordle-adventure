use wordle_adventure::{dictionary, Alphabet, LetterState};

use std::{io, io::Write};

fn main() -> Result<(), io::Error> {
    let alphabet = Alphabet::default();
    let answer = dictionary::choose_random_word();
    println!("answer {}", answer);

    let mut attempts = 1;

    while attempts < 7 {
        println!();

        for letter in &alphabet {
            match letter.state() {
                LetterState::Unused => print!("{}", **letter),
                LetterState::Missed => print!("_"),
                LetterState::Almost => print!("?"),
                LetterState::Exact => print!("!"),
            }
            print!(" ");
        }
        io::stdout().flush()?;

        print!("\nguess #{}: ", attempts);
        io::stdout().flush()?;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        if guess == answer {
            println!("you got it!");
            return Ok(());
        }

        println!("wrong, try again.");
        attempts += 1;
    }

    println!("\nsorry about your luck. it was '{}'.", answer);
    Ok(())
}
