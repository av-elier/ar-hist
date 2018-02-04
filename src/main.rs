#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

mod ar_http;

fn main() {
    println!("Hello, world!");
    let r = ar_http::do_http();
    println!("{:?}", r);
}

// #[derive(Debug)]
// struct PrintableResult(Result<(), Box<Error>>);
