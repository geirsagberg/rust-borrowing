use rust_borrowing::kitchen_supplies::*;
use rust_borrowing::*;

pub fn main() {
    let egg = Egg;
    let flour = Flour;
    let sugar = Sugar;
    let cake = make_cake(vec![egg], flour, sugar);
    eat(cake);
}
