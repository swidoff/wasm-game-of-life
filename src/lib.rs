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
    cells: [FixedBitSet; 2],
    i: usize,
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
        self.cells[self.i].as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = [
            Universe::new_cells(width, self.height),
            Universe::new_cells(width, self.height)
        ];
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = [
            Universe::new_cells(self.width, height),
            Universe::new_cells(self.width, height)
        ];
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[self.i].toggle(idx);
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let cells = &self.cells[self.i];
        let nw = self.get_index(north, west);
        count += cells[nw] as u8;

        let n = self.get_index(north, column);
        count += cells[n] as u8;

        let ne = self.get_index(north, east);
        count += cells[ne] as u8;

        let w = self.get_index(row, west);
        count += cells[w] as u8;

        let e = self.get_index(row, east);
        count += cells[e] as u8;

        let sw = self.get_index(south, west);
        count += cells[sw] as u8;

        let s = self.get_index(south, column);
        count += cells[s] as u8;

        let se = self.get_index(south, east);
        count += cells[se] as u8;

        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let new_i = (self.i + 1) % 2;
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[self.i].contains(idx);
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
                self.cells[new_i].set(idx, next_cell);
            }
        }

        self.i = new_i;
    }

    pub fn empty(width: u32, height: u32) -> Universe {
        log!("Creating an empty universe of width {} and height {}", width, height);
        utils::set_panic_hook();
        let cells = [
            Universe::new_cells(width, height),
            Universe::new_cells(width, height)
        ];
        Universe { width, height, cells, i: 0 }
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
        for idx in 0..self.cells[self.i].len() {
            self.cells[self.i].set(idx, Math::random() < prob)
        }
    }

    pub fn clear(&mut self) {
        self.cells[self.i].clear()
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
                self.cells[self.i].set(index, value)
            }
        }
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells[self.i]
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[self.i].set(idx, true);
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &(0..((self.width * self.height) as usize)).chunks(self.width as usize) {
            for index in line {
                let symbol = if self.cells[self.i].contains(index) { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}
