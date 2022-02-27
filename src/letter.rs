use std::ops::Deref;

/// A game-state letter that keeps track of user attempts involving this single letter
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Letter {
    c: char,
    state: State,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Unused,
    Missed,
    Almost,
    Exact,
}

impl Letter {
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl From<char> for Letter {
    fn from(c: char) -> Self {
        Self {
            c: c.to_ascii_lowercase(),
            state: Default::default(),
        }
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
