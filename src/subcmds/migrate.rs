use clap::ArgMatches;
use std::error::Error;
use super::super::ar_filter;
use super::super::ar_pg;

pub fn call(matches: ArgMatches) -> Result<(), Box<Error>> {
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
