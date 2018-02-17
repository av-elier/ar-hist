use clap::ArgMatches;
use std::error::Error;
use serde_json;
use super::super::ar_pg;
use super::super::ar_types;
use chrono::{DateTime, Utc};

pub fn call(matches: ArgMatches) -> Result<(), Box<Error>> {
    let pg = ar_pg::ArPg::new()?;
    let pg_table_orig = matches
        .value_of("pg-table-orig")
        .expect("no --pg-table-orig value");

    call_internal(pg, pg_table_orig)
}

fn call_internal(pg: ar_pg::ArPg, table: &str) -> Result<(), Box<Error>> {
    let orig: Vec<(String, String)> = pg.get_kv_merged(table)?;
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
    lifetime_percent: f64,
    cat_id: i32,
    cat_title: String,
    views: i32,
    votes_total: i32,
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
                lifetime_percent: x.lifetime_percent(),
                cat_id: x.category.id,
                cat_title: x.category.title.clone(),
                views: x.watch_count,
                votes_positive: x.positive,
                votes_negative: x.negative,
                votes_total: x.positive + x.negative,
                shares_all: x.statistic.sum(),
                status: x.status.to_string(),
                title: str::replace(&x.title, ",", " ЗПТ"),
            })
            .collect::<Vec<Essential>>())
    }

    fn csv_header() -> String {
        "time,time_utc,lifetime_percent,id,cat_id,cat_title,views,votes_positive,votes_negative,votes_total,shares_all,status,title"
            .to_string()
    }
    fn csv_line(&self) -> String {
        format!(
            "{time},{time_utc},{lifetime_percent},{id},{cat_id},{cat_title},{views},{votes_positive},{votes_negative},{votes_total},{shares_all},{status},{title}",
            time = self.time,
            time_utc = self.time.timestamp(),
            id = self.id,
            lifetime_percent = self.lifetime_percent,
            cat_id = self.cat_id,
            cat_title = self.cat_title,
            views = self.views,
            votes_positive = self.votes_positive,
            votes_negative = self.votes_negative,
            votes_total = self.votes_total,
            shares_all = self.shares_all,
            status = self.status,
            title = self.title
        )
    }
}
