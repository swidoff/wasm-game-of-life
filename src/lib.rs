extern crate web_sys;

use std::fmt;

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use js_sys::Math;
use wasm_bindgen::__rt::core::fmt::Formatter;
use wasm_bindgen::prelude::*;

mod utils;
mod timer;

use timer::Timer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,

}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = Universe::new_cells(width, self.height)
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = Universe::new_cells(self.width, height)
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += if self.cells.contains(idx) { 1 } else { 0 };
            }
        }

        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells.contains(idx);
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn empty(width: u32, height: u32) -> Universe {
        log!("Creating an empty universe of width {} and height {}", width, height);
        utils::set_panic_hook();
        let cells = Universe::new_cells(width, height);
        Universe { width, height, cells }
    }

    fn new_cells(width: u32, height: u32) -> FixedBitSet {
        FixedBitSet::with_capacity((width * height) as usize)
    }

    pub fn random(width: u32, height: u32, prob: f64) -> Universe {
        utils::set_panic_hook();
        let mut universe = Universe::empty(width, height);
        universe.shuffle(prob);
        universe
    }

    pub fn shuffle(&mut self, prob: f64) {
        for idx in 0..self.cells.len() {
            self.cells.set(idx, Math::random() < prob)
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear()
    }

    pub fn add_space_ship(&mut self, row: u32, col: u32) {
        let space_ship = [
            false, true, false, false, true,
            true, false, false, false, false,
            true, false, false, false, true,
            true, true, true, true, false,
        ];

        self.insert_at(row, col, &space_ship[..], 5, 4);
    }

    pub fn add_pulsar(&mut self, row: u32, col: u32) {
        let pulsar = [
            false, false, true, true, true, false, false, false, true, true, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            false, false, true, true, true, false, false, false, true, true, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, true, false, false, false, true, true, true, false, false,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            true, false, false, false, false, true, false, true, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, true, false, false, false, true, true, true, false, false,
        ];

        self.insert_at(row, col, &pulsar[..], 13, 13);
    }


    pub fn add_glider(&mut self, row: u32, col: u32) {
        let glider = [
            false, true, false,
            false, false, true,
            true, true, true,
        ];

        self.insert_at(row, col, &glider[..], 3, 3);
    }


    fn insert_at(&mut self, row: u32, col: u32, img: &[bool], width: u32, height: u32) {
        for r in 0..height {
            for c in 0..width {
                let img_index = (r * width + c) as usize;
                let value = img[img_index];
                let index = self.get_index((row + r) % self.height, (col + c) % self.width);
                self.cells.set(index, value)
            }
        }
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &(0..((self.width * self.height) as usize)).chunks(self.width as usize) {
            for index in line {
                let symbol = if self.cells.contains(index) { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}
