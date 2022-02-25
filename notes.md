# Wordle Adventure

This is a step-by-step adventure for learning Rust by building out a Wordle game. 

1. Project Setup
    ```shell
    $ cargo new wordle-adventure
    $ cd wordle-adventure
    $ cargo run
    ```

2. Choose random word

    * [Steal Dictionary from Wordle](https://uk.pcmag.com/games/138290/want-to-up-your-wordle-game-the-winning-word-is-right-on-the-page)
    * `include_str!` macro - static string compiled into code
    * add `rand` crate (there's no `std::rand`)
    * `str::lines()` - docs are your friend
    * why `unwrap` here? We've done our bounds checking. It's safe.

3. Basic game loop

    * `stdin` & `stdout` - docs are your friend
    * `Result` and `?`
    * `starts_with` - `guess == answer` would work but have to strip `\n` from guess
    * `print!`, `println!` macros

4. Alphabet
