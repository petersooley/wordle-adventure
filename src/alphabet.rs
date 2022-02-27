use crate::Letter;

const LEN: usize = 26;
pub struct Alphabet([Letter; LEN]);

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
