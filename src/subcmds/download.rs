use clap::ArgMatches;
use std::error::Error;
use serde;
use serde_json;
use std;
use super::super::ar_types;
use super::super::ar_http;
use super::super::ar_store;

pub fn call(matches: ArgMatches<'static>) -> Result<(), Box<Error>> {
    if matches.is_present("typed") {
        download_initiatives::<ar_types::Initiative>(matches)?;
    } else {
        download_initiatives::<serde_json::Value>(matches)?;
    }
    Ok(())
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
