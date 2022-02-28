use crate::{Letter, LetterState, Word};

const LEN: usize = 26;
pub struct Alphabet([Letter; LEN]);

impl Alphabet {
    pub fn update(&mut self, guess: &Word) {
        for letter in guess {
            let index = (**letter as usize) - 97;
            let old_state = self.0[index].state();
            match letter.state() {
                LetterState::Unused => {}
                LetterState::Exact => self.0[index].set_state(LetterState::Exact),
                LetterState::Almost => {
                    if matches!(old_state, LetterState::Unused | LetterState::Missed) {
                        self.0[index].set_state(LetterState::Almost)
                    }
                }
                LetterState::Missed => {
                    if matches!(old_state, LetterState::Unused) {
                        self.0[index].set_state(LetterState::Missed)
                    }
                }
            }
        }
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        let letters: [Letter; LEN] = (b'a'..=b'z')
            .map(|b| Letter::from(char::from(b)))
            .collect::<Vec<Letter>>()
            .try_into()
            .unwrap();

        Self(letters)
    }
}

impl<'a> IntoIterator for &'a Alphabet {
    type Item = &'a Letter;
    type IntoIter = std::slice::Iter<'a, Letter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_slice().into_iter()
    }
}
