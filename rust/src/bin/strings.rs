struct StringBuilder {
    buffer: Vec<u8>,
}

impl StringBuilder {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    fn append(&mut self, s: &str) {
        self.buffer.extend(s.bytes());
    }
    fn to_string(&self) -> String {
        String::from_utf8(self.buffer.clone()).unwrap()
    }
}

fn main() {
    let str = "Hello, world!";
}
