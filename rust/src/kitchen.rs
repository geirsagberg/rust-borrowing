use std::any::Any;

struct Cake;
struct Egg;
struct Flour;
struct Sugar;

fn make_cake(eggs: Vec<Egg>, flour: Flour, sugar: Sugar) -> Cake {
    Cake
}

fn eat(_: impl Any) {
    println!("Yum!");
}

pub fn do_kitchen_stuff() {
    let eggs = vec![Egg, Egg];
    let flour = Flour;
    let sugar = Sugar;
    let cake = make_cake(eggs, flour, sugar);
    eat(cake);
}
