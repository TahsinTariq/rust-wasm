mod utils;

extern crate fixedbitset;
extern crate js_sys;
extern crate web_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (((row + self.height) % self.height) * self.width + ((column + self.width) % self.width))
            as usize
    }
    fn get_index_no_overhead(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };
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
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true)
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = {
            let _timer = Timer::new("allocate new cells");
            self.cells.clone()
        };
        {
            let _timer = Timer::new("iterative generetion");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index_no_overhead(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
                    // log!(
                    //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    //     row,
                    //     col,
                    //     cell,
                    //     live_neighbors
                    // );
                    let next_cell = match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    };
                    next.set(idx, next_cell);
                    // self.cells.set(idx, next_cell);
                    // log!("    it becomes {:?}", next_cell);
                }
            }
        }
        let _timer = Timer::new("replace old cells");
        self.cells = next;
    }

    #[allow(clippy::new_without_default)]
    pub fn new(width: u32, height: u32) -> Universe {
        // pub fn new() -> Universe {
        //     let width = 1;
        //     let height = 2;
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    #[allow(clippy::new_without_default)]
    pub fn new_empty(width: u32, height: u32) -> Universe {
        // pub fn new() -> Universe {
        //     let width = 1;
        //     let height = 2;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| 0).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| 0).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx)
    }

    pub fn add_glider(&mut self, row: u32, col: u32) {
        self.set_cells(&[
            (row, col),
            (row + 1, col),
            (row + 2, col),
            (row, col - 1),
            (row + 1, col - 2),
        ]);
    }

    pub fn gosper_gun(&mut self, row: u32, col: u32) {
        let v = vec![
            (row, col),
            (row + 1, col - 2),
            (row + 2, col - 1),
            (row + 2, col),
            (row + 3, col),
            (row + 2, col + 1),
            (row + 1, col + 2),
            (row - 1, col - 3),
            (row - 2, col - 3),
            (row - 3, col - 2),
            (row - 4, col - 1),
            (row - 4, col),
            (row - 4, col + 1),
            (row - 3, col + 2),
            (row - 2, col + 3),
            (row - 1, col + 3),
            (row - 13, col),
            (row - 14, col),
            (row - 13, col - 1),
            (row - 14, col - 1),
            (row + 6, col - 1),
            (row + 6, col - 2),
            (row + 6, col - 3),
            (row + 7, col - 1),
            (row + 7, col - 2),
            (row + 7, col - 3),
            (row + 8, col),
            (row + 10, col),
            (row + 10, col + 1),
            (row + 8, col - 4),
            (row + 10, col - 4),
            (row + 10, col - 5),
            (row + 20, col - 2),
            (row + 20, col - 3),
            (row + 21, col - 2),
            (row + 21, col - 3),
        ];
        self.set_cells(&v);
    }

    pub fn add_pulser(&mut self, row: u32, col: u32) {
        let v = vec![
            (row + 1, col + 2),
            (row + 1, col + 3),
            (row + 1, col + 4),
            (row + 1, col - 2),
            (row + 1, col - 3),
            (row + 1, col - 4),
            (row - 1, col + 2),
            (row - 1, col + 3),
            (row - 1, col + 4),
            (row - 1, col - 2),
            (row - 1, col - 3),
            (row - 1, col - 4),
            (row + 6, col + 2),
            (row + 6, col + 3),
            (row + 6, col + 4),
            (row + 6, col - 2),
            (row + 6, col - 3),
            (row + 6, col - 4),
            (row - 6, col + 2),
            (row - 6, col + 3),
            (row - 6, col + 4),
            (row - 6, col - 2),
            (row - 6, col - 3),
            (row - 6, col - 4),
            (row + 2, col + 1),
            (row + 3, col + 1),
            (row + 4, col + 1),
            (row - 2, col + 1),
            (row - 3, col + 1),
            (row - 4, col + 1),
            (row + 2, col - 1),
            (row + 3, col - 1),
            (row + 4, col - 1),
            (row - 2, col - 1),
            (row - 3, col - 1),
            (row - 4, col - 1),
            (row + 2, col + 6),
            (row + 3, col + 6),
            (row + 4, col + 6),
            (row - 2, col + 6),
            (row - 3, col + 6),
            (row - 4, col + 6),
            (row + 2, col - 6),
            (row + 3, col - 6),
            (row + 4, col - 6),
            (row - 2, col - 6),
            (row - 3, col - 6),
            (row - 4, col - 6),
        ];
        self.set_cells(&v);
    }

    pub fn set_random(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }
    pub fn ded(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }
    }
}

use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
