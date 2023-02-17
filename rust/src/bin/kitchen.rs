use rust_borrowing::{eat, inspect};

struct Cake;
struct Egg;
struct Flour;
struct Sugar;

fn make_cake(_eggs: Vec<Egg>, _flour: Flour, _sugar: Sugar) -> Cake {
    Cake
}

pub fn main() {
    let eggs = vec![Egg, Egg];
    inspect(&eggs);
    let flour = Flour;
    let sugar = Sugar;
    let cake = make_cake(eggs, flour, sugar);
    eat(cake);
}
