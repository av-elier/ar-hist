#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::error::Error;
use serde_json::Value;

mod ar_http;

fn main() {
    println!("Hello, world!");

    let r: Result<Vec<Value>, Box<Error>> = ar_http::get_ar_json_vec();

    match r {
        Ok(initiatives) => println!("got {:?} initiatives", initiatives.len()),
        Err(e) => {
            println!("failed to get initiatives {:?}", e);
            std::process::exit(1);
        },
    }
}
