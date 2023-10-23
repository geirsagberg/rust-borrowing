#![allow(unused_variables, dead_code)]

#[derive(Debug)]
pub struct Egg;

#[derive(Debug)]
pub struct Flour;

#[derive(Debug)]
pub struct Sugar;

#[derive(Debug)]
pub struct Cake;

pub fn make_cake(eggs: Vec<Egg>, flour: Flour, sugar: Sugar) -> Cake {
    Cake
}
