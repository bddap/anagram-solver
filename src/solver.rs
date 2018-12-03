use enum_map::EnumMap;
use std::ops::Index;

#[derive(Enum, Copy, Clone, Debug)]
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

    pub fn from_bytes(other: &[u8]) -> Option<Vec<Letter>> {
        other.iter().map(Letter::from_byte).collect()
    }

    pub fn inc(self) -> Option<Letter> {
        match self {
            Letter::A => Some(Letter::B),
            Letter::B => Some(Letter::C),
            Letter::C => Some(Letter::D),
            Letter::D => Some(Letter::E),
            Letter::E => Some(Letter::F),
            Letter::F => Some(Letter::G),
            Letter::G => Some(Letter::H),
            Letter::H => Some(Letter::I),
            Letter::I => Some(Letter::J),
            Letter::J => Some(Letter::K),
            Letter::K => Some(Letter::L),
            Letter::L => Some(Letter::M),
            Letter::M => Some(Letter::N),
            Letter::N => Some(Letter::O),
            Letter::O => Some(Letter::P),
            Letter::P => Some(Letter::Q),
            Letter::Q => Some(Letter::R),
            Letter::R => Some(Letter::S),
            Letter::S => Some(Letter::T),
            Letter::T => Some(Letter::U),
            Letter::U => Some(Letter::V),
            Letter::V => Some(Letter::W),
            Letter::W => Some(Letter::X),
            Letter::X => Some(Letter::Y),
            Letter::Y => Some(Letter::Z),
            Letter::Z => None,
        }
    }
}

pub struct LetterCounts(EnumMap<Letter, usize>);

impl LetterCounts {
    pub fn count_letters(word: &[Letter]) -> LetterCounts {
        let mut ret = enum_map!{
            Letter::A => 0,
            _ => 0,
        };
        for letter in word {
            ret[*letter] += 1;
        }
        LetterCounts(ret)
    }

    pub fn checked_sub(&self, other: &LetterCounts) -> Option<LetterCounts> {
        let mut ret = EnumMap::new();
        for (key, val) in self.0 {
            ret[key] = val.checked_sub(other.0[key])?;
        }
        Some(LetterCounts(ret))
    }
}

impl Default for LetterCounts {
    fn default() -> LetterCounts {
        LetterCounts(enum_map!{_ => 0})
    }
}

impl Index<Letter> for LetterCounts {
    type Output = usize;

    fn index(&self, index: Letter) -> &usize {
        &self.0[index]
    }
}

pub struct CountedWord {
    word: Vec<Letter>,
    counts: LetterCounts,
}

impl CountedWord {
    pub fn from_word(word: Vec<Letter>) -> CountedWord {
        let counts = LetterCounts::count_letters(&word);
        CountedWord { word, counts }
    }

    pub fn from_words(mut words: Vec<Vec<Letter>>) -> Vec<CountedWord> {
        words
            .drain(..)
            .map(|word| CountedWord::from_word(word))
            .collect()
    }
}

pub struct WordList<'a> {
    sorted_lists: EnumMap<Letter, Vec<&'a CountedWord>>,
}

impl<'a> WordList<'a> {
    pub fn new(words: &'a [CountedWord]) -> WordList<'a> {
        let sorted_lists = enum_map!{
            letter => WordList::sorted_list_for(words, letter),
        };
        WordList {
            sorted_lists,
        }
    }

    fn sorted_list_for(words: &'a [CountedWord], letter: Letter) -> Vec<&'a CountedWord> {
        let mut ret: Vec<&CountedWord> = words.iter().filter(|cw| cw.counts[letter] != 0).collect();
        ret.sort_by(|a, b| a.counts[letter].cmp(&b.counts[letter]));
        ret
    }
}

pub fn find<'a>(wl: &WordList, target: LetterCounts) -> Option<Vec<Vec<Letter>>> {
    _find(wl, Letter::A, target).map(|vec| {
        vec.iter()
            .map(|counted_word| counted_word.word.clone())
            .collect()
    })
}

fn _find<'a>(
    wl: &'a WordList,
    letter: Letter,
    target: LetterCounts,
) -> Option<Vec<&'a CountedWord>> {
    if target[letter] == 0 {
        // go to next letter
        // incrementing past Z returns none
        // none means we found an anagram
        return match letter.inc() {
            Some(next_letter) => _find(wl, next_letter, target),
            None => Some(Vec::new()),
        };
    }

    for word in wl.sorted_lists[letter]
        .iter()
        .take_while(|w| w.counts[letter] <= target[letter])
    {
        match target.checked_sub(&word.counts) {
            Some(new_target) => match _find(wl, letter, new_target) {
                Some(mut vec) => {
                    vec.push(word);
                    return Some(vec);
                }
                None => {}
            },
            None => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{find, CountedWord, Letter, LetterCounts, WordList};
    use english_words::get_words;

    #[test]
    fn find_word() {
        let words = get_words();
        let counted_words = CountedWord::from_words(words);
        let word_list = WordList::new(&counted_words);
        let letters = Letter::from_bytes(b"racecarracecar").unwrap();
        let letter_counts = LetterCounts::count_letters(&letters);
        find(&word_list, letter_counts).unwrap();
    }
}
