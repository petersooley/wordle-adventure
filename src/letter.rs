use std::ops::Deref;

/// A game-state letter that keeps track of user attempts involving this single letter
#[derive(Debug)]
pub struct Letter {
    c: char,
    state: State,
}

#[derive(Debug)]
pub enum State {
    Unused,
    Missed,
    Almost,
    Exact,
}

impl Letter {
    pub fn new(c: char) -> Self {
        Self {
            c: c.to_ascii_lowercase(),
            state: Default::default(),
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Unused
    }
}

impl Deref for Letter {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
