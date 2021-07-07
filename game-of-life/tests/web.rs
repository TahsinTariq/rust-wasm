//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

// #[cfg(test)]
// pub fn input_spaceship() -> Universe {
//     let mut universe = Universe::new(6, 6);
//     // let mut universe = Universe::new();
//     // universe.set_width(6);
//     // universe.set_height(6);
//     universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
//     universe
// }

// #[cfg(test)]
// pub fn expected_spaceship() -> Universe {
//     let mut universe = Universe::new(6, 6);
//     // let mut universe = Universe::new();
//     // universe.set_width(6);
//     // universe.set_height(6);
//     universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
//     universe
// }

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new_empty(5, 5);
    // let mut universe = Universe::new();
    // universe.set_width(6);
    // universe.set_height(6);
    // universe.set_cells(&[(1, 2), (2, 2), (3, 2)]);
    universe.set_cells(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new_empty(5, 5);
    // let mut universe = Universe::new();
    // universe.set_width(6);
    // universe.set_height(6);
    // universe.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    universe.set_cells(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
    universe
}

#[wasm_bindgen_test]
pub fn setting() {
    let mut u1 = Universe::new_empty(2, 2);
    // let mut u1 = Universe::new(6,6);
    // u1.set_height(1);
    // u1.set_width(2);
    u1.set_cells(&[(0, 0), (0, 1)]);
    unsafe {
        // println!("{}", std::ptr::read(u1.get_cells()));
        // assert_eq!(format!("{:b}", std::ptr::read(u1.get_cells())), "11");
        assert_eq!(std::ptr::read(u1.get_cells()), 3);
        // assert_ne!(std::ptr::read(u1.get_cells()), 0);
    }
}
#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    // input_universe.tick();
    // std::ptr::read
    unsafe {
        assert_eq!(
            std::ptr::read(input_universe.get_cells()),
            std::ptr::read(expected_universe.get_cells())
        );
    }
}

#[wasm_bindgen_test]
pub fn test_toogle() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = Universe::new_empty(12, 12);

    // This is what our spaceship should look like
    // after one tick in our universe.
    let mut expected_universe = Universe::new_empty(12, 12);
    input_universe.toggle_cell(0, 0);
    expected_universe.set_cells(&[(0, 0)]);

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    // input_universe.tick();
    // std::ptr::read
    unsafe {
        assert_eq!(
            std::ptr::read(input_universe.get_cells()),
            std::ptr::read(expected_universe.get_cells())
        );
    }
}
