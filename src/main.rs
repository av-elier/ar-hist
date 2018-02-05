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
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::error::Error;

mod ar_http;
mod ar_save;

fn main() {
    env_logger::init();
    let matches = App::new("ar-hist")
       .version("0.0.1")
       .author("av_elier")
       .arg(Arg::with_name("save")
            .long("save")
            .takes_value(true)
            .possible_values(&["postgres", "redis"])
            .help("Disables saving to db"))
       .arg(Arg::with_name("ar-status")
            .long("ar-status")
            .takes_value(true)
            .possible_values(&["active", "attention", "completed", "considered", "implemented"])
            .help("Disables saving to db"))
       .get_matches();

    let result = get_ar_initiatives(matches);

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

fn get_ar_initiatives(matches: ArgMatches)-> Result<(), Box<Error>> {
    info!("Getting ar initiatives");

    let initiatives = ar_http::get_ar_json_vec(matches.value_of("ar-status"))?;
    info!("got {:?} initiatives", initiatives.len());

    match matches.value_of("save") {
        Some("postgres") => { ar_save::save_initiatives_to_postgres(initiatives)?; info!("save to postgres succeed")},
        Some("redis") => { ar_save::save_initiatives_to_redis(initiatives)?; info!("save to redis succeed")},
        _ => info!("save to db SKIPPED"),
    }

    Ok(())
}
