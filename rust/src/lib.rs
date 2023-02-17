use std::any::Any;

pub fn eat(_: impl Any) {
    println!("Yum!");
}

pub fn inspect(_: &impl Any) {
    println!("Inspecting...");
}
