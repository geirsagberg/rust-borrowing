use std::alloc::{alloc, dealloc, Layout};
use std::{slice, str};

fn print_hello() {
    let layout = Layout::from_size_align(5, 1).unwrap();
    let pointer = unsafe { alloc(layout) };
    if !pointer.is_null() {
        let hello = unsafe { slice::from_raw_parts_mut(pointer, 5) };
        hello[0] = b'H';
        hello[1] = b'e';
        hello[2] = b'l';
        hello[3] = b'l';
        hello[4] = b'o';
        println!("{}", unsafe { str::from_utf8_unchecked(hello) });
        unsafe {
            dealloc(pointer, layout);
        }
    }
}

fn safe_hello() {
    let hello = "Hello TDC!";
    println!("{}", hello);
}

fn safe_hello_mutation() {
    let mut hello = String::from("Hello");
    hello += " TDC!";
    println!("{}", hello);
}

fn main() {
    print_hello();
    safe_hello();
    safe_hello_mutation();
}
