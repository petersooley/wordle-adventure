use crate::Letter;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Word([Letter; 5]);

impl FromStr for Word {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [Letter::from('-'); 5];
        let mut i = 0;
        for c in s.chars() {
            if i > 4 {
                break;
            }
            out[i] = c.into();
            i += 1;
        }
        if i < 5 {
            return Err(format!("failed to create a full word"));
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

#[rustfmt::skip]
#[cfg(test)]
mod word_test {
    use super::*;

    #[test]
    fn parse_words() {
        assert_eq!("hello".parse::<Word>().unwrap(), Word::from(['h', 'e', 'l', 'l', 'o']));
        assert!("world".parse::<Word>().is_ok());
        assert!("oops".parse::<Word>().is_err());
        assert!("surewhynot".parse::<Word>().is_ok());
        assert_eq!("surewhynot".parse::<Word>().unwrap(), Word::from(['s', 'u', 'r', 'e', 'w']));
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
        assert_eq!("hello\nworld", Word::from(['h', 'e', 'l', 'l', 'o']));
        assert_eq!(String::from("hello"), Word::from(['h', 'e', 'l', 'l', 'o']));
    }
}
