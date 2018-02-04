#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::error::Error;

mod ar_http;

fn main() {
    let result = get_ar_initiatives();
    match result{
        Ok(_) => {
            println!("success");
            std::process::exit(0)
        },
        Err(e) => {
            println!("error {:?}", e);
            std::process::exit(1)
        },
    }
}

fn get_ar_initiatives()-> Result<(), Box<Error>> {
    println!("Getting ar initiatives");

    let initiatives = ar_http::get_ar_json_vec()?;

    println!("got {:?} initiatives", initiatives.len());
    Ok(())
}
