#![feature(iterator_try_fold)]
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate postgres;
extern crate redis;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

use clap::ArgMatches;
use std::error::Error;

mod ar_types;
mod ar_http;
mod ar_store;
mod ar_filter;
mod ar_pg;
mod cli;

fn main() {
    env_logger::init();
    let app = cli::ar_hist_app();
    let matches = app.get_matches();

    let result = match matches.subcommand {
        Some(subcmd) => match subcmd.name.as_ref() {
            "download" => if subcmd.matches.is_present("typed") {
                download_initiatives::<ar_types::Initiative>(subcmd.matches)
            } else {
                download_initiatives::<serde_json::Value>(subcmd.matches)
            },
            "migrate" => migrate_initiatives(subcmd.matches),
            _ => panic!("impossible cli"),
        },
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

fn download_initiatives<T>(matches: ArgMatches<'static>) -> Result<(), Box<Error>>
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
            let table_name = matches.value_of("pg-table-name").expect("");
            ar_store::save_initiatives_to_postgres(initiatives, table_name)?;
            info!("save to postgres succeed")
        }
        Some("redis") => {
            ar_store::save_initiatives_to_redis(initiatives)?;
            info!("save to redis succeed")
        }
        Some("stdout") => println!("{:?}", initiatives),
        _ => info!("save to db SKIPPED"),
    }

    Ok(())
}

fn migrate_initiatives(matches: ArgMatches) -> Result<(), Box<Error>> {
    let pg = ar_pg::ArPg::new()?;
    let pg_table_orig = matches
        .value_of("pg-table-orig")
        .expect("no --pg-table-orig value");
    let pg_table_dest = matches.value_of("pg-table-dest");
    Ok(match matches.value_of("action").expect("") {
        "filter-unchanged" => {
            debug!("migrate filter-unchanged - selecting...");
            let orig: Vec<(String, String)> = pg.get_kv_postgres(pg_table_orig)?;
            debug!("migrate filter-unchanged - filtering...");
            let filtered = ar_filter::filter_spahshots(orig)?;
            if let Some(pg_table_dest) = pg_table_dest {
                debug!("migrate filter-unchanged - saving...");
                pg.set_kv_postgres(pg_table_dest, filtered)?;
            } else {
                debug!("migrate filter-unchanged - printing...");
                for (k, v) in filtered {
                    println!("{:?}: {:?}", k, v);
                }
            }
        }
        _ => panic!("unknown cli"),
    })
}
