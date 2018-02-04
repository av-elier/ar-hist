use redis;
use redis::Commands;
use std::env;
use std::error::Error;
use serde_json;
use chrono;


pub fn save_initiatives_to_redis(ins: Vec<serde_json::Value>) -> Result<(), Box<Error>> {
    let redis_url = match env::var("REDIS_URL") {
        Ok(x) => x,
        _ => "redis://localhost:6379/".to_string(), // local
    };

    let client = redis::Client::open(redis_url.as_str())?;
    let con = client.get_connection()?;

    let key = chrono::offset::Utc::now().to_rfc3339();
    let text: String = serde_json::to_string(&ins)?;
    let _: () = con.set(key, text)?;

    Ok(())
}
