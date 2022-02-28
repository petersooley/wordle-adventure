use wordle_adventure::{dictionary, Alphabet, LetterState, Word};

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

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let mut guess: Word = match buf.trim_end().parse() {
            Ok(w) => w,
            Err(e) => {
                println!("invalid guess: {}", e);
                continue;
            }
        };

        let is_correct = guess.check(&answer);

        if is_correct {
            println!("you got it!");
            return Ok(());
        }

        println!("wrong, try again.");
        attempts += 1;
    }

    println!("\nsorry about your luck. it was '{}'.", answer);
    Ok(())
}
