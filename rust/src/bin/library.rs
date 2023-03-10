use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    content: String,
}

impl Book {
    fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

struct Library {
    books_by_title: HashMap<String, Book>,
}

struct BookLover {
    loaned_books: Vec<Book>,
}

fn read(book: &Book) {
    println!("Reading {}...", book.title);
    println!("{}", book.content);
}

fn main() {
    let book = Book::new("The Rust Programming Language", "...");
    read(&book);
}
