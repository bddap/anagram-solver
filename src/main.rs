mod english_words;
mod solver;

#[macro_use]
extern crate enum_map;

use solver::{find, CountedWord, Letter, LetterCounts, WordList};
use english_words::get_words;

fn main() {
    let words = get_words();
    let counted_words = CountedWord::from_words(words);
    let word_list = WordList::new(&counted_words);
    let letters = Letter::from_bytes(b"abcdefghijklmnopqrstuvwxyz").unwrap();
    let letter_counts = LetterCounts::count_letters(&letters);
    let anagram = find(&word_list, letter_counts).unwrap();
    println!("{:?}", anagram);
}
