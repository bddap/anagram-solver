use enum_map::EnumMap;
use std::ops::Index;

#[derive(Enum, Copy, Clone)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Letter {
    pub fn from_byte(cha: &u8) -> Option<Letter> {
        let mut cha = *cha;
        if cha >= b'A' && cha <= b'Z' {
            cha = cha - b'A' + b'a';
        }
        match cha {
            b'a' => Some(Letter::A),
            b'b' => Some(Letter::B),
            b'c' => Some(Letter::C),
            b'd' => Some(Letter::D),
            b'e' => Some(Letter::E),
            b'f' => Some(Letter::F),
            b'g' => Some(Letter::G),
            b'h' => Some(Letter::H),
            b'i' => Some(Letter::I),
            b'j' => Some(Letter::J),
            b'k' => Some(Letter::K),
            b'l' => Some(Letter::L),
            b'm' => Some(Letter::M),
            b'n' => Some(Letter::N),
            b'o' => Some(Letter::O),
            b'p' => Some(Letter::P),
            b'q' => Some(Letter::Q),
            b'r' => Some(Letter::R),
            b's' => Some(Letter::S),
            b't' => Some(Letter::T),
            b'u' => Some(Letter::U),
            b'v' => Some(Letter::V),
            b'w' => Some(Letter::W),
            b'x' => Some(Letter::X),
            b'y' => Some(Letter::Y),
            b'z' => Some(Letter::Z),
            _ => None,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            Letter::A => b'a',
            Letter::B => b'b',
            Letter::C => b'c',
            Letter::D => b'd',
            Letter::E => b'e',
            Letter::F => b'f',
            Letter::G => b'g',
            Letter::H => b'h',
            Letter::I => b'i',
            Letter::J => b'j',
            Letter::K => b'k',
            Letter::L => b'l',
            Letter::M => b'm',
            Letter::N => b'n',
            Letter::O => b'o',
            Letter::P => b'p',
            Letter::Q => b'q',
            Letter::R => b'r',
            Letter::S => b's',
            Letter::T => b't',
            Letter::U => b'u',
            Letter::V => b'v',
            Letter::W => b'w',
            Letter::X => b'x',
            Letter::Y => b'y',
            Letter::Z => b'z',
        }
    }

    pub fn from_bytes(other: &[u8]) -> Option<Vec<Letter>> {
        other.iter().map(Letter::from_byte).collect()
    }
}

struct LetterCounts(EnumMap<Letter, u32>);

impl LetterCounts {
    pub(crate) fn count_letters(word: &[Letter]) -> LetterCounts {
        let mut ret = enum_map!{
            Letter::A => 0,
            _ => 0,
        };
        for letter in word {
            ret[*letter] += 1;
        }
        LetterCounts(ret)
    }

    pub(crate) fn checked_sub(&self, other: &LetterCounts) -> Option<LetterCounts> {
        let mut ret = EnumMap::new();
        for (key, val) in self.0 {
            ret[key] = val.checked_sub(other.0[key])?;
        }
        Some(LetterCounts(ret))
    }
}

impl Index<Letter> for LetterCounts {
    type Output = u32;

    fn index(&self, index: Letter) -> &u32 {
        &self.0[index]
    }
}

struct CountedWord {
    word: Vec<Letter>,
    counts: LetterCounts,
}

impl CountedWord {
    pub(crate) fn from_word(word: Vec<Letter>) -> CountedWord {
        let counts = LetterCounts::count_letters(&word);
        CountedWord { word, counts }
    }

    fn from_words(mut words: Vec<Vec<Letter>>) -> Vec<CountedWord> {
        words
            .drain(..)
            .map(|word| CountedWord::from_word(word))
            .collect()
    }
}

struct WordList<'a> {
    words: &'a [CountedWord],
    sorted_lists: EnumMap<Letter, Vec<&'a CountedWord>>,
}

impl<'a> WordList<'a> {
    pub fn new(words: &'a [CountedWord]) -> WordList<'a> {
        let sorted_lists = enum_map!{
            letter => WordList::sorted_list_for(words, letter),
        };
        WordList {
            words,
            sorted_lists,
        }
    }

    fn sorted_list_for(words: &'a [CountedWord], letter: Letter) -> Vec<&'a CountedWord> {
        let mut ret: Vec<&CountedWord> = words.iter().filter(|cw| cw.counts[letter] != 0).collect();
        ret.sort_by(|a, b| a.counts[letter].cmp(&b.counts[letter]));
        ret
    }

    pub fn find_anagrams(&self, lc: &LetterCounts) -> Option<Vec<&'a [Letter]>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{CountedWord, Letter, LetterCounts, WordList};
    use english_words::get_words;

    #[test]
    fn find_word() {
        let words = get_words();
        let counted_words = CountedWord::from_words(words);
        let word_list = WordList::new(&counted_words);
        let letters = Letter::from_bytes(b"racecarracecar").unwrap();
        let letter_counts = LetterCounts::count_letters(&letters);
        word_list.find_anagrams(&letter_counts).unwrap();
    }
}
