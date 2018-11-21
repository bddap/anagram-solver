extern crate libflate;

use english_words::libflate::gzip::Decoder;
use std::io::Read;

pub fn get_words() -> Vec<Vec<::solver::Letter>> {
    let list = include_bytes!("words_alpha.txt.gz");
    let mut decoder = Decoder::new(&list[..]).unwrap();
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();
    (&buf)
        .split(|c| *c == b'\n')
        .map(|slice| {
            slice
                .iter()
                .map(|b| ::solver::Letter::from_byte(b).unwrap())
                .collect()
        }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_words() {
        super::get_words();
    }
}
