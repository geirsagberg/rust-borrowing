#![allow(unused_variables, dead_code)]

pub mod kitchen_supplies;

use std::fmt::Debug;

/// Moves a value into the function, consuming it.
pub fn eat(something: impl Debug) {
    println!("{:?}! Yum!", something);
}

/// Moves a value into the function, then returns it.
pub fn handle<T: Debug>(something: T) -> T {
    println!("Handling {:?}...", something);
    return something;
}

/// Borrows a reference to something for inspection purposes.
pub fn inspect(something: &impl Debug) {
    println!("Inspecting {:?}...", something);
}
