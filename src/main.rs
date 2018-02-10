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

mod ar_types;
mod ar_http;
mod ar_store;
mod ar_filter;
mod ar_pg;
mod cli;

mod subcmds;

fn main() {
    env_logger::init();
    let app = cli::ar_hist_app();
    let matches = app.get_matches();

    let result = match matches.subcommand {
        Some(subcmd) => match subcmd.name.as_ref() {
            "download" => subcmds::download::call(subcmd.matches),
            "migrate" => subcmds::migrate::call(subcmd.matches),
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
