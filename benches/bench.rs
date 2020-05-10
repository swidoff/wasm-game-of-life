#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    #[bench]
    fn universe_ticks(b: &mut test::Bencher) {
        let mut universe = wasm_game_of_life::Universe::empty(128, 128);
        b.iter(|| universe.tick());
    }
}