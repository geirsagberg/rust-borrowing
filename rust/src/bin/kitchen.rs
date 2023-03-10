use rust_borrowing::*;

#[derive(Debug)]
struct Cake;

#[derive(Debug)]
struct Egg;

#[derive(Debug)]
struct Flour;

#[derive(Debug)]
struct Sugar;

fn make_cake(_eggs: Vec<Egg>, _flour: Flour, _sugar: Sugar) -> Cake {
    Cake
}

pub fn main() {}
