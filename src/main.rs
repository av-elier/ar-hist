#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;
extern crate redis;
extern crate chrono;
extern crate postgres;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;

mod ar_http;
mod ar_save;

fn main() {
    env_logger::init();
    let result = get_ar_initiatives();
    match result{
        Ok(_) => {
            info!("success");
            std::process::exit(0)
        },
        Err(e) => {
            error!("error {:?}", e);
            std::process::exit(1)
        },
    }
}

fn get_ar_initiatives()-> Result<(), Box<Error>> {
    info!("Getting ar initiatives");

    let initiatives = ar_http::get_ar_json_vec()?;
    info!("got {:?} initiatives", initiatives.len());

    ar_save::save_initiatives_to_postgres(initiatives)?;
    info!("saving to db succeed");

    Ok(())
}
