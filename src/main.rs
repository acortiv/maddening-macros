// use maddening_macros::mm_assert_ne;

use maddening_macros::{FoldToDisplay, fold_to_display_for};

fn main() {
    println!("Hello, world!");
    // ************************************************
    // # A quick custom implementation of assert_ne
    // mm_assert_ne!(3, 3);
    // assert_ne!(3, 3);
    // ************************************************
    let a = "Hello";
    let b = String::from(" World! ");
    let c = 1312026;
    println!(fold_to_display_for!(a, b, c));
}
