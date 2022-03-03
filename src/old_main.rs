use wordle_adventure::{dictionary, Alphabet, Letter, LetterState, Word};

use std::{io, io::Write};

fn show_letter_states<'a>(letters: impl IntoIterator<Item = &'a Letter>) {
    let mut chars = String::new();
    let mut states = String::new();
    for letter in letters {
        chars.push(**letter);
        chars.push(' ');
        match letter.state() {
            LetterState::Unused => states.push_str(". "),
            LetterState::Missed => states.push_str("× "),
            LetterState::Almost => states.push_str("! "),
            LetterState::Exact => states.push_str("✓ "),
        }
    }
    println!("{}", chars);
    println!("{}", states);
}

fn main() -> Result<(), io::Error> {
    let mut alphabet = Alphabet::default();
    let answer = dictionary::choose_random_word();
    println!("answer {}", answer);

    let mut attempts = 1;

    while attempts < 7 {
        println!();
        show_letter_states(&alphabet);

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

        if !dictionary::is_in_list(&guess) {
            println!("that is not a word in the list");
            continue;
        }

        let is_correct = guess.check(&answer);

        if is_correct {
            println!("you got it!");
            return Ok(());
        }

        alphabet.update(&guess);

        println!();

        show_letter_states(&guess);

        attempts += 1;
    }

    println!("\nsorry about your luck. it was '{}'.", answer);
    Ok(())
}
