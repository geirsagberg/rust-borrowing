fn main() {
    let a = "abc".to_string();
    let mut b = a;
    b += "def";
    let c = "abc";
    let d = c;
    println!("c: {}", d);
    println!("c: {}", d);
    // println!("a: {}", a);
    println!("b: {}", b);
}
