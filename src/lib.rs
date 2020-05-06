mod utils;

extern crate web_sys;

use web_sys::console;
use wasm_bindgen::prelude::*;
use std::fmt;
use wasm_bindgen::__rt::core::fmt::Formatter;
use itertools::Itertools;
use js_sys::Math;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
        let cells = FixedBitSet::with_capacity((width * height) as usize);
        Universe { width, height, cells }
    }

    pub fn random(width: u32, height: u32, prob: f64) -> Universe {
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);
        for idx in 0..(width * height) as usize {
            cells.set(idx, Math::random() < prob)
        }
        Universe { width, height, cells }
    }

    pub fn add_space_ship(&mut self, row: u32, col: u32) {
        let space_ship = [
            [false, true, false, false, true],
            [true, false, false, false, false],
            [true, false, false, false, true],
            [true, true, true, true, false],
        ];

        let coords = (0..4).cartesian_product(0..5)
            .map(|(r, c)| {
                let index = self.get_index((row + r) % self.height, (col + c) % self.width);
                let cell = space_ship[r as usize][c as usize];
                (index, cell)
            })
            .collect_vec();

        for &(i, cell) in coords.iter() {
            self.cells.set(i, cell);
        }
    }

    pub fn render(&self) -> String {
        return self.to_string();
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
