#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate postgres;
extern crate redis;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use clap::ArgMatches;
use std::error::Error;

mod ar_http;
mod ar_save;
mod cli;

fn main() {
    env_logger::init();
    let matches = cli::ar_hist_app().get_matches();

    let result = match matches.subcommand {
        Some(subcmd) => {
            if subcmd.matches.is_present("typed") {
                get_ar_initiatives::<Vec<ar_http::ArStruct>>(subcmd.matches)
            } else {
                get_ar_initiatives::<Vec<serde_json::Value>>(subcmd.matches)
            }
        }
        _ => Ok(error!("unknown cli")),
    };

    match result {
        Ok(_) => {
            info!("success");
            std::process::exit(0)
        }
        Err(e) => {
            error!("error {:?}", e);
            std::process::exit(1)
        }
    }
}

fn get_ar_initiatives<T>(matches: ArgMatches<'static>) -> Result<(), Box<Error>>
where
    for<'de> T: serde::Deserialize<'de> + 'static,
    for<'de> T: serde::Serialize,
    T: std::fmt::Debug,
{
    info!("Getting ar initiatives");

    let initiatives: Vec<T> = ar_http::get_ar_json_vec(matches.value_of("ar-status"))?;
    info!("got {:?} initiatives", initiatives.len());

    match matches.value_of("save") {
        Some("postgres") => {
            ar_save::save_initiatives_to_postgres(initiatives)?;
            info!("save to postgres succeed")
        }
        Some("redis") => {
            ar_save::save_initiatives_to_redis(initiatives)?;
            info!("save to redis succeed")
        }
        Some("stdout") => println!("{:?}", initiatives),
        _ => info!("save to db SKIPPED"),
    }

    Ok(())
}
