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

pub fn main2() {
    // Owned values on the stack
    let flour = Flour;
    let sugar = Sugar;
    // Owned value on the heap
    let eggs = vec![Egg, Egg];
    // Borrowed value
    inspect(&eggs);
    inspect(&flour);
    // Move values into the function and return a new value
    let cake = make_cake(eggs, flour, sugar);
    // Consume the value
    eat(cake);
}

struct Cheese;

pub fn main() {
    let owned_cheese = Cheese;
    let borrowed_cheese = &owned_cheese;
    let mut moved_cheese = owned_cheese;
    let mut mutably_borrowed_cheese = &mut moved_cheese;
}
