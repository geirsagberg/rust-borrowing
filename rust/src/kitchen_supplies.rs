#[derive(Debug)]
pub struct Cake;

#[derive(Debug)]
pub struct Egg;

#[derive(Debug)]
pub struct Flour;

#[derive(Debug)]
pub struct Sugar;

pub fn make_cake(_eggs: Vec<Egg>, _flour: Flour, _sugar: Sugar) -> Cake {
    Cake
}
