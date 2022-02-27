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

4. `Word` struct

    * motivation: 
      * only compare first 5 characters (instead of `guess.starts_with(answer)`)
      * newtypes are useful for extending functionality to existing types
    * move dictionary to lib
    * create `Word` as newtype for `[5, char]`
    * `impl FromStr` to convert from `&str` to `Word` easily (with `parse`)
    * `impl PartialEq`
    * `impl Display`
    * tests
    * update `choose_random_word`

5. `Alphabet` & `Letter`
   * motivation: want to print the alphabet and keep track of user guesses
   * `Letter` design choice: why not stick to one enum i.e. `Letter::Almost("d")`?
   * `Alphabet` design choice: why not one variant-per-letter enum i.e. `Alphabet::D(State::Almost)`?
   * ranges: `b'a'..=b'z'`
   * char literal: `'a'` vs str literal: `"a"`
   * `IntoIterator` for Alphabet - hello lifetimes! (we want to return an iterator over the items without giving up ownership)
   * `for letter in &alphabet` - notice the immutable borrow. needed because in a loop and can't _move_ it more than once.
   * `Deref` trait and `**`

6. Update `Word` struct to use `Letter` instead of char
   * motivation: this allows us to use `Word` for managing the state of user guesses
   * `From` trait implementations makes things a little clearer
   * `Copy` & `Clone` traits are fine since a `Letter` isn't holding a lot of memory
7. 
