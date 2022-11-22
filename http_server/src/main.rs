fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];

    dbg!(&string);
    dbg!(string_slice);

    let server = Server::new(string);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    fn run(self) {

    }
}