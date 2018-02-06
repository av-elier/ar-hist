use redis;
use redis::Commands;
use std::env;
use std::error::Error;
use serde_json;
use chrono;
use postgres;

fn get_key_and_value(ins: Vec<serde_json::Value>) -> Result<(String, String), Box<Error>> {
    let date = chrono::offset::Utc::now().to_rfc3339();
    let text: String = serde_json::to_string(&ins)?;
    Ok((date, text))
}

pub fn save_initiatives_to_redis(ins: Vec<serde_json::Value>) -> Result<(), Box<Error>> {
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

pub fn save_initiatives_to_postgres(ins: Vec<serde_json::Value>) -> Result<(), Box<Error>> {
    let postgres_url = env::var("DATABASE_URL")?;
    let conn = postgres::Connection::connect(postgres_url, postgres::TlsMode::None).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS kv (
                    k   VARCHAR PRIMARY KEY,
                    v   VARCHAR NOT NULL
                  )",
        &[],
    )?;

    let (k, v) = get_key_and_value(ins)?;

    conn.execute("INSERT INTO kv (k, v) VALUES ($1, $2)", &[&k, &v])?;
    Ok(())
}
