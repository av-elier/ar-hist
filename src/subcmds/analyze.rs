use clap::ArgMatches;
use std::error::Error;
use serde_json;
use super::super::ar_pg;
use super::super::ar_types;
use chrono::{DateTime, Duration, Utc};
use std::ops::Sub;

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

    essentials.drain_filter(|x| x.time.lt(&Utc::now().sub(Duration::hours(24 * 15))));

    essentials.sort_by(|x, y| x.time.cmp(&y.time));

    add_weekago_info(&mut essentials);

    essentials.drain_filter(|x| x.time.lt(&Utc::now().sub(Duration::hours(24 * 8))));

    println!("{}", Essential::csv_header());
    for e in essentials.iter() {
        println!("{}", e.csv_line());
    }
    Ok(())
}

fn add_weekago_info(essentials: &mut Vec<Essential>) -> () {
    let esss_clone = essentials.clone();
    for i in 0..essentials.len() {
        let mut ess = &mut essentials[i];
        let ess_time = ess.time;
        for j in (0..i).rev() {
            let ess_wa = &esss_clone[j];
            if ess_wa.id != ess.id {
                continue;
            }
            let wa_time = ess_wa.time;
            let duration_since = ess_time.signed_duration_since(wa_time);
            if duration_since.gt(&Duration::hours(24 * 6 + 20)) {
                ess.with_weekagos(&ess_wa);
                break;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Essential {
    time: DateTime<Utc>,
    id: i32,
    lifetime_percent: f64,
    cat_id: i32,
    cat_title: String,
    user_id: i32,
    user_fullname: String,
    views: i32,
    votes_total: i32,
    votes_positive: i32,
    votes_negative: i32,
    shares_all: i32,
    comments_count: i32,
    status: String,
    title: String,
    views_weekagos: i32,
    votes_total_weekagos: i32,
    votes_positive_weekagos: i32,
    votes_negative_weekagos: i32,
    shares_all_weekagos: i32,
    comments_count_weekagos: i32,
}

impl Essential {
    fn new(
        time: DateTime<Utc>,
        id: i32,
        lifetime_percent: f64,
        cat_id: i32,
        cat_title: String,
        user_id: i32,
        user_fullname: String,
        views: i32,
        votes_total: i32,
        votes_positive: i32,
        votes_negative: i32,
        shares_all: i32,
        comments_count: i32,
        status: String,
        title: String,
    ) -> Essential {
        Essential {
            time: time,
            id: id,
            lifetime_percent: lifetime_percent,
            cat_id: cat_id,
            cat_title: cat_title,
            user_id: user_id,
            user_fullname: user_fullname,
            views: views,
            votes_total: votes_total,
            votes_positive: votes_positive,
            votes_negative: votes_negative,
            shares_all: shares_all,
            comments_count: comments_count,
            status: status,
            title: title,
            views_weekagos: views,
            votes_total_weekagos: votes_total,
            votes_positive_weekagos: votes_positive,
            votes_negative_weekagos: votes_negative,
            shares_all_weekagos: shares_all,
            comments_count_weekagos: comments_count,
        }
    }
    fn with_weekagos(&mut self, other: &Essential) -> () {
        self.views_weekagos = self.views - other.views;
        self.votes_total_weekagos = self.votes_total - other.votes_total;
        self.votes_positive_weekagos = self.votes_positive - other.votes_positive;
        self.votes_negative_weekagos = self.votes_negative - other.votes_negative;
        self.shares_all_weekagos = self.shares_all - other.shares_all;
        self.comments_count_weekagos = self.comments_count - other.comments_count;
    }

    fn parse(k: String, v: String) -> Result<Vec<Essential>, Box<Error>> {
        // debug!("parsing Essential from {} {:?}", k, v);
        let time = DateTime::parse_from_rfc3339(&k)?.with_timezone(&Utc);
        let initiative: Vec<ar_types::Initiative> = serde_json::from_str(&v)?;

        Ok(initiative
            .iter()
            .map(|x| {
                Essential::new(
                    time,
                    x.id,
                    x.lifetime_percent(),
                    x.category.id,
                    x.category.title.clone(),
                    x.user.id,
                    x.user.fullname.clone(),
                    x.watch_count,
                    x.positive + x.negative,
                    x.positive,
                    x.negative,
                    x.statistic.sum(),
                    x.comments_count,
                    x.status.to_string(),
                    str::replace(&x.title, ",", " ЗПТ"),
                )
            })
            .collect::<Vec<Essential>>())
    }

    fn csv_header() -> String {
        "time,time_utc,lifetime_percent,id,cat_id,cat_title,user_id,user_fullname\
         ,views,votes_positive,votes_negative,votes_total,shares_all,comments_count\
         ,status,title,views_weekagos,votes_total_weekagos\
         ,votes_positive_weekagos,votes_negative_weekagos,shares_all_weekagos,comments_count_weekagos"
            .to_string()
    }
    fn csv_line(&self) -> String {
        format!(
            "{time},{time_utc},{lifetime_percent},{id},{cat_id},{cat_title},{user_id},{user_fullname}\
            ,{views},{votes_positive},{votes_negative},{votes_total},{shares_all},{comments_count}\
            ,{status},{title},{views_weekagos},{votes_total_weekagos}\
            ,{votes_positive_weekagos},{votes_negative_weekagos},{shares_all_weekagos},{comments_count_weekagos}",
            time = self.time,
            time_utc = self.time.timestamp(),
            id = self.id,
            lifetime_percent = self.lifetime_percent,
            cat_id = self.cat_id,
            cat_title = self.cat_title,
            user_id = self.user_id,
            user_fullname = self.user_fullname,
            views = self.views,
            votes_positive = self.votes_positive,
            votes_negative = self.votes_negative,
            votes_total = self.votes_total,
            shares_all = self.shares_all,
            comments_count = self.comments_count,
            status = self.status,
            title = self.title,
            views_weekagos = self.views_weekagos,
            votes_total_weekagos = self.votes_total_weekagos,
            votes_positive_weekagos = self.votes_positive_weekagos,
            votes_negative_weekagos = self.votes_negative_weekagos,
            shares_all_weekagos = self.shares_all_weekagos,
            comments_count_weekagos = self.comments_count_weekagos
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;
    use chrono::{Duration, Utc};
    use std::ops::Sub;
    #[test]
    fn test_add_weekago_info() {
        let _ = env_logger::try_init();
        let last = Essential::new(
            Utc::now(),
            1,
            0f64,
            1,
            "".to_string(),
            1,
            "Вася".to_string(),
            3,
            7,
            4,
            3,
            10,
            1,
            "".to_string(),
            "test".to_string(),
        );
        let mut essentials = vec![
            Essential::new(
                Utc::now().sub(Duration::hours(7 * 24 + 1)),
                1,
                0f64,
                1,
                "".to_string(),
                1,
                "Вася".to_string(),
                1,
                3,
                2,
                1,
                4,
                2,
                "".to_string(),
                "test".to_string(),
            ),
            Essential::new(
                Utc::now().sub(Duration::hours(24)),
                1,
                0f64,
                1,
                "".to_string(),
                1,
                "Вася".to_string(),
                2,
                5,
                3,
                2,
                6,
                3,
                "".to_string(),
                "test".to_string(),
            ),
            last.clone(),
        ];
        add_weekago_info(&mut essentials);
        assert_eq!(
            last.eq(&essentials[2]),
            false,
            "{:?}\n==\n{:?}",
            last,
            &essentials[2]
        );
    }
}
