mod dictionary;

use std::{io, io::Write};

fn main() -> Result<(), io::Error> {
    let answer = dictionary::choose_random_word();
    // println!("answer {}", answer);
    let mut attempts = 1;

    while attempts < 7 {
        print!("\nguess #{}: ", attempts);
        io::stdout().flush()?;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        if guess.starts_with(answer) {
            println!("you got it!");
            return Ok(())
        }

        println!("wrong, try again.");
        attempts += 1;
    }

    println!("\nsorry about your luck. it was '{}'.", answer);
    Ok(())
}
