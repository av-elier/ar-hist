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

mod ar_types;
mod ar_http;
mod ar_save;
mod cli;

fn main() {
    env_logger::init();
    let app = cli::ar_hist_app();
    let matches = app.get_matches();

    let result = match matches.subcommand {
        Some(subcmd) => {
            if subcmd.name == "download" {
                if subcmd.matches.is_present("typed") {
                    get_ar_initiatives::<ar_types::Initiative>(subcmd.matches)
                } else {
                    get_ar_initiatives::<serde_json::Value>(subcmd.matches)
                }
            } else {
                panic!("unknown cli")
            }
        }
        _ => panic!("unknown cli"),
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
            let table_name = matches
                .value_of("pg-table-name")
                .expect("it supposed to be required arg");
            ar_save::save_initiatives_to_postgres(initiatives, table_name)?;
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
