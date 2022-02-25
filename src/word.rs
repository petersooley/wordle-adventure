use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Word([char; 5]);

impl FromStr for Word {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = ['-'; 5];
        let mut i = 0;
        for c in s.chars() {
            if i > 4 {
                break;
            }
            out[i] = c;
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
        write!(f, "{}", self.0.iter().collect::<String>())
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

#[cfg(test)]
mod word_test {
    use super::*;

    #[test]
    fn parse_words() {
        assert_eq!(
            "hello".parse::<Word>().unwrap(),
            Word(['h', 'e', 'l', 'l', 'o'])
        );
        assert!("world".parse::<Word>().is_ok());
        assert!("oops".parse::<Word>().is_err());
        assert!("surewhynot".parse::<Word>().is_ok());
        assert_eq!(
            "surewhynot".parse::<Word>().unwrap(),
            Word(['s', 'u', 'r', 'e', 'w'])
        );
    }

    #[test]
    fn display_words() {
        assert_eq!("hello", Word(['h', 'e', 'l', 'l', 'o']).to_string());
    }

    #[test]
    fn compare_words() {
        assert_eq!("hello", Word(['h', 'e', 'l', 'l', 'o']));
        assert_ne!("world", Word(['h', 'e', 'l', 'l', 'o']));
        assert_ne!("hel", Word(['h', 'e', 'l', 'l', 'o']));
        assert_eq!("hello\nworld", Word(['h', 'e', 'l', 'l', 'o']));
        assert_eq!(String::from("hello"), Word(['h', 'e', 'l', 'l', 'o']));
    }
}
