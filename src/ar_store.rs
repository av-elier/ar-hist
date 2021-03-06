use redis;
use redis::Commands;
use std::env;
use std::error::Error;
use serde::Serialize;
use serde_json;
use chrono;
use postgres;

fn get_key_and_value<T>(ins: Vec<T>) -> Result<(String, String), Box<Error>>
where
    T: Serialize,
{
    let date = chrono::offset::Utc::now().to_rfc3339();
    let text: String = serde_json::to_string(&ins)?;
    Ok((date, text))
}

pub fn save_initiatives_to_redis<T>(ins: Vec<T>) -> Result<(), Box<Error>>
where
    T: Serialize,
{
    let redis_url = match env::var("REDIS_URL") {
        Ok(x) => x,
        _ => "redis://localhost:6379/".to_string(), // local
    };

    let client = redis::Client::open(redis_url.as_str())?;
    let con = client.get_connection()?;

    let (k, v) = get_key_and_value(ins)?;
    let _: () = con.set(k, v)?;

    Ok(())
}

pub fn save_initiatives_to_postgres<T>(ins: Vec<T>, table_name: &str) -> Result<(), Box<Error>>
where
    T: Serialize,
{
    let postgres_url = env::var("DATABASE_URL")?;
    let conn = postgres::Connection::connect(postgres_url, postgres::TlsMode::None)?;
    conn.execute(
        format!(
            "CREATE TABLE IF NOT EXISTS {} (
                    k   VARCHAR PRIMARY KEY,
                    v   VARCHAR NOT NULL
                  )",
            table_name
        ).as_str(),
        &[],
    )?;

    let (k, v) = get_key_and_value(ins)?;

    conn.execute("INSERT INTO kv (k, v) VALUES ($1, $2)", &[&k, &v])?;
    Ok(())
}
