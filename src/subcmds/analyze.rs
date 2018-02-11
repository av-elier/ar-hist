use clap::ArgMatches;
use std::error::Error;
use serde_json;
use super::super::ar_pg;
use super::super::ar_types;
use chrono::DateTime;
use chrono::offset::Utc;

pub fn call(matches: ArgMatches) -> Result<(), Box<Error>> {
    let pg = ar_pg::ArPg::new()?;
    let pg_table_orig = matches
        .value_of("pg-table-orig")
        .expect("no --pg-table-orig value");

    call_internal(pg, pg_table_orig)
}

fn call_internal(pg: ar_pg::ArPg, table: &str) -> Result<(), Box<Error>> {
    let orig: Vec<(String, String)> = pg.get_kv_postgres(table)?;
    let mut essentials: Vec<Essential> = vec![];
    for &(ref k, ref v) in orig.iter() {
        let mut esss = Essential::parse(k.to_string(), v.to_string())?;
        essentials.append(&mut esss);
    }
    println!("{}", Essential::csv_header());
    for e in essentials.iter() {
        println!("{}", e.csv_line());
    }
    Ok(())
}

struct Essential {
    time: DateTime<Utc>,
    id: i32,
    views: i32,
    votes_positive: i32,
    votes_negative: i32,
    shares_all: i32,
    status: String,
    title: String,
}

impl Essential {
    fn parse(k: String, v: String) -> Result<Vec<Essential>, Box<Error>> {
        // debug!("parsing Essential from {} {:?}", k, v);
        let time = DateTime::parse_from_rfc3339(&k)?.with_timezone(&Utc);
        let initiative: Vec<ar_types::Initiative> = serde_json::from_str(&v)?;

        Ok(initiative
            .iter()
            .map(|x| Essential {
                time: time,
                id: x.id,
                views: x.watch_count,
                votes_positive: x.positive,
                votes_negative: x.negative,
                shares_all: x.statistic.sum(),
                status: x.status.to_string(),
                title: str::replace(&x.title, ",", " ЗПТ"),
            })
            .collect::<Vec<Essential>>())
    }

    fn csv_header() -> String {
        "time,id,views,votes_positive,votes_negative,shares_all,status,title".to_string()
    }
    fn csv_line(&self) -> String {
        format!(
            "{time},{id},{views},{votes_positive},{votes_negative},{shares_all},{status},{title}",
            time = self.time,
            id = self.id,
            views = self.views,
            votes_positive = self.votes_positive,
            votes_negative = self.votes_negative,
            shares_all = self.shares_all,
            status = self.status,
            title = self.title
        )
    }
}
