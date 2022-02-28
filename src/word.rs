use crate::{Error, Letter, LetterState};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Word([Letter; 5]);

impl Word {
    pub fn check(&mut self, answer: &Self) -> bool {
        // keep track characters we've seen while looping. we need this for handling repeated chars.
        let mut seen: HashMap<char, usize> = HashMap::with_capacity(5);

        let exact_matches = self.0.iter_mut().enumerate().fold(0, |count, (i, l)| {
            let seen_count = if let Some(seen_count) = seen.get_mut(&**l) {
                *seen_count += 1;
                *seen_count
            } else {
                seen.insert(**l, 1);
                1_usize
            };

            let answer_letter = answer[i];
            if *answer_letter == **l {
                l.set_state(LetterState::Exact);
                return count + 1;
            }

            match answer.count_matches(*l) {
                0 => l.set_state(LetterState::Missed),
                matches_count => {
                    if seen_count <= matches_count {
                        l.set_state(LetterState::Almost);
                    } else {
                        l.set_state(LetterState::Missed);
                    }
                }
            }

            count
        });

        exact_matches == 5
    }

    fn count_matches(&self, needle: Letter) -> usize {
        self.0
            .iter()
            .fold(0, |c, l| if **l == *needle { c + 1 } else { c })
    }
}

impl FromStr for Word {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [Letter::from('-'); 5];
        let mut i = 0;
        for c in s.chars() {
            if i > 4 {
                return Err(Error::WordTooBig);
            }
            out[i] = c.into();
            i += 1;
        }
        if i < 5 {
            return Err(Error::WordTooSmall);
        }
        Ok(Self(out))
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|l| **l).collect::<String>())
    }
}

impl PartialEq<Word> for &str {
    fn eq(&self, other: &Word) -> bool {
        if let Ok(ref w) = self.parse::<Word>() {
            w == other
        } else {
            false
        }
    }
}

impl PartialEq<Word> for String {
    fn eq(&self, other: &Word) -> bool {
        &self.as_str() == other
    }
}

impl From<[char; 5]> for Word {
    fn from(chars: [char; 5]) -> Self {
        Word(chars.map(Letter::from))
    }
}

impl Index<usize> for Word {
    type Output = Letter;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a> IntoIterator for &'a Word {
    type Item = &'a Letter;
    type IntoIter = std::slice::Iter<'a, Letter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_slice().into_iter()
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod word_test {
    use super::*;

    #[test]
    fn parse_words() {
        assert_eq!("hello".parse::<Word>().unwrap(), Word::from(['h', 'e', 'l', 'l', 'o']));
        assert!("world".parse::<Word>().is_ok());
        assert!("oops".parse::<Word>().is_err());
        assert!("surewhynot".parse::<Word>().is_err());
        assert_eq!("surew".parse::<Word>().unwrap(), Word::from(['s', 'u', 'r', 'e', 'w']));
    }

    #[test]
    fn display_words() {
        assert_eq!("hello", Word::from(['h', 'e', 'l', 'l', 'o']).to_string());
    }

    #[test]
    fn compare_words() {
        assert_eq!("hello", Word::from(['h', 'e', 'l', 'l', 'o']));
        assert_ne!("world", Word::from(['h', 'e', 'l', 'l', 'o']));
        assert_ne!("hel", Word::from(['h', 'e', 'l', 'l', 'o']));
        assert_ne!("hello\nworld", Word::from(['h', 'e', 'l', 'l', 'o']));
        assert_eq!(String::from("hello"), Word::from(['h', 'e', 'l', 'l', 'o']));
    }

    #[test]
    fn count_chars() {
        assert_eq!(2, Word::from(['h', 'e', 'l', 'l', 'o']).count_matches('l'.into()));
        assert_eq!(1, Word::from(['h', 'e', 'l', 'l', 'o']).count_matches('h'.into()));
        assert_eq!(0, Word::from(['h', 'e', 'l', 'l', 'o']).count_matches('d'.into()));
    }

    #[test]
    fn check_guesses() {
        let answer: Word = "hello".parse().unwrap();

        let mut correct = answer.clone();
        assert!(correct.check(&answer));
        let correct_result = Word([
            Letter::new('h', LetterState::Exact),
            Letter::new('e', LetterState::Exact),
            Letter::new('l', LetterState::Exact),
            Letter::new('l', LetterState::Exact),
            Letter::new('o', LetterState::Exact),
        ]);
        assert_eq!(correct_result, correct);

        let mut guess1: Word = "happy".parse().unwrap();
        assert!(!guess1.check(&answer));
        let guess1_result = Word([
            Letter::new('h', LetterState::Exact),
            Letter::new('a', LetterState::Missed),
            Letter::new('p', LetterState::Missed),
            Letter::new('p', LetterState::Missed),
            Letter::new('y', LetterState::Missed),
        ]);

        assert_eq!(guess1_result, guess1);

        // two 'l' chars, both get the "almost"
        let mut guess2: Word = "shell".parse().unwrap();
        assert!(!guess2.check(&answer));
        let guess2_result = Word([
            Letter::new('s', LetterState::Missed),
            Letter::new('h', LetterState::Almost),
            Letter::new('e', LetterState::Almost),
            Letter::new('l', LetterState::Exact),
            Letter::new('l', LetterState::Almost),
        ]);

        assert_eq!(guess2_result, guess2);

        // two 'e' chars, only one gets the "almost"
        let mut guess3: Word = "sleet".parse().unwrap();
        assert!(!guess3.check(&answer));
        let guess3_result = Word([
            Letter::new('s', LetterState::Missed),
            Letter::new('l', LetterState::Almost),
            Letter::new('e', LetterState::Almost),
            Letter::new('e', LetterState::Missed),
            Letter::new('t', LetterState::Missed),
        ]);

        assert_eq!(guess3_result, guess3);
    }
}
