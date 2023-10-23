#![allow(unused_variables, dead_code)]

use rust_borrowing::kitchen_supplies::*;
use rust_borrowing::*;

pub fn main() {
    let egg = Egg;
    inspect(&egg);
    let egg = handle(egg);
    let flour = Flour;
    let sugar = Sugar;
    let cake = make_cake(vec![egg], flour, sugar);
    eat(cake);
}
