# Rust borrowing

- What is Rust?
- Why should I use Rust?

## Value vs reference

- Value ~= owned
- Reference ~= borrowed

## Garbage collection

- Rust does not have garbage collection
- Rust has ownership
- Rust has borrowing

## Memory handling strategies

- Garbage collection
- Manual memory management
- Reference counting
- Ownership

## Ownership

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Borrowing

- Variables can be borrowed as a reference (&T).
- A reference is like a pointer, but guaranteed to be valid.
- You can have multiple borrows, but only one owner.
- When the owner goes out of scope, the value will be dropped.

## Borrowing rules

1. Any borrow must last for a scope no greater than that of the owner.
2. You may have one or the other of these two kinds of borrows, but not both at the same time:
   - One or more references (&T) to a resource.
   - Exactly one mutable reference (&mut T)

## Pass by value

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x);
}
```

```csharp
static void Main(string[] args)
{
    int x = 5;
    int y = x;
    y += 1;
    Console.WriteLine(x);
}
```

## Pass by reference

```rust
fn main() {
    let mut x = 5;
    add_one(&mut x);
    println!("{}", x);
}

fn add_one(x: &mut i32) {
    *x += 1;
}
```

```csharp
static void Main(string[] args)
{
    int x = 5;
    AddOne(ref x);
    Console.WriteLine(x);
}

static void AddOne(ref int x)
{
    x += 1;
}
```
