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
        Some(subcmd) => get_ar_initiatives(subcmd.matches),
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

fn get_ar_initiatives<'a>(matches: ArgMatches<'a>) -> Result<(), Box<Error>> {
    info!("Getting ar initiatives");

    let initiatives: Vec<serde_json::Value> =
        ar_http::get_ar_json_vec(matches.value_of("ar-status"))?;
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
        _ => info!("save to db SKIPPED"),
    }

    Ok(())
}
