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

    pub fn dec(self) -> Option<Letter> {
        match self {
            Letter::A => None,
            Letter::B => Some(Letter::A),
            Letter::C => Some(Letter::B),
            Letter::D => Some(Letter::C),
            Letter::E => Some(Letter::D),
            Letter::F => Some(Letter::E),
            Letter::G => Some(Letter::F),
            Letter::H => Some(Letter::G),
            Letter::I => Some(Letter::H),
            Letter::J => Some(Letter::I),
            Letter::K => Some(Letter::J),
            Letter::L => Some(Letter::K),
            Letter::M => Some(Letter::L),
            Letter::N => Some(Letter::M),
            Letter::O => Some(Letter::N),
            Letter::P => Some(Letter::O),
            Letter::Q => Some(Letter::P),
            Letter::R => Some(Letter::Q),
            Letter::S => Some(Letter::R),
            Letter::T => Some(Letter::S),
            Letter::U => Some(Letter::T),
            Letter::V => Some(Letter::U),
            Letter::W => Some(Letter::V),
            Letter::X => Some(Letter::W),
            Letter::Y => Some(Letter::X),
            Letter::Z => Some(Letter::Y),
        }
    }
}

struct LetterCounts(EnumMap<Letter, u32>);

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

    pub fn add(&self, other: &LetterCounts) -> LetterCounts {
        LetterCounts(enum_map!{
            k => self.0[k] + other.0[k],
        })
    }

    pub fn exceeds(&self, other: &LetterCounts) -> bool {
        for (key, val) in self.0 {
            if val < other.0[key] {
                return true;
            }
        }
        false
    }
}

impl Default for LetterCounts {
    fn default() -> LetterCounts {
        LetterCounts(enum_map!{_ => 0})
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

    pub fn find_anagrams(&'a self, lc: &'a LetterCounts) -> AnagramSearch<'a> {
        AnagramSearch::new(self, lc)
    }
}

struct AnagramSearch<'a> {
    wl: &'a WordList<'a>,
    phrase: Vec<&'a CountedWord>,       // Word stack
    phrase_letter_counts: LetterCounts, // Always represents the letter counts in phrase
    target: &'a LetterCounts,
    progress: EnumMap<Letter, usize>,
}

impl<'a> AnagramSearch<'a> {
    fn new(wl: &'a WordList<'a>, target: &'a LetterCounts) -> AnagramSearch<'a> {
        AnagramSearch {
            wl,
            phrase: vec![],
            phrase_letter_counts: LetterCounts::default(),
            target,
            progress: enum_map!{_ => 0},
        }
    }

    pub fn find_anagrams(&mut self) -> Option<Vec<&'a [Letter]>> {}

    fn inc(&mut self, low_letter: Letter) -> Option<()> {
        let sorted_list = self.wl.sorted_lists[low_letter];
        let room = self.target.checked_sub(&self.phrase_letter_counts).expect(
            "It shouldn't be possible for a phrase to have less than zero of a certain letter.",
        );

        let next_index = self.progress[low_letter] + 1;
        if next_index >= sorted_list.len()
            || sorted_list[next_index].counts[low_letter] > room[low_letter]
        {
            self.progress[low_letter] = 0;
            self.inc(low_letter.dec()?)
        } else {
            self.progress[low_letter] = next_index;
            Some(())
        }
    }

    fn push_word(&mut self, word: &'a CountedWord) -> Option<()> {
        let potential_count = self.phrase_letter_counts.add(&word.counts);
        if potential_count.exceeds(self.target) {
            None
        } else {
            self.phrase.push(word);
            self.phrase_letter_counts = potential_count;
            Some(())
        }
    }

    fn pop_word(&mut self) -> Option<()> {
        let word = self.phrase.pop()?;
        self.phrase_letter_counts = self.phrase_letter_counts.checked_sub(&word.counts).expect(
            "It shouldn't be possible for a phrase to have less than zero of a certain letter.",
        );
        Some(())
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
