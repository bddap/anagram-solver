#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn find_one(b: &mut Bencher) {}

    #[bench]
    fn find_several(b: &mut Bencher) {}

    #[bench]
    fn find_all(b: &mut Bencher) {}
}
