use redis;
use redis::Commands;
use std::env;
use std::error::Error;
use serde::Serialize;
use serde_json;
use chrono;
use postgres;
use openssl::ssl::{SslConnector, SslConnectorBuilder, SslMethod, SSL_VERIFY_NONE};

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

pub struct ArPg {
    conn: postgres::Connection,
}

impl ArPg {
    pub fn new() -> Result<ArPg, Box<Error>> {
        let postgres_url = env::var("DATABASE_URL")?;

        let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
        builder.set_verify(SSL_VERIFY_NONE);
        let conn: SslConnector = builder.build();
        let pg_connector = postgres::tls::openssl::OpenSsl::from(conn);

        info!("about to connect to postgres");
        let conn =
            postgres::Connection::connect(postgres_url, postgres::TlsMode::Prefer(&pg_connector))?;
        info!("connected to postgres succeed! {:?}", conn);
        Ok(ArPg { conn: conn })
    }

    pub fn get_kv_postgres(&self, table: &str) -> Result<Vec<(String, String)>, Box<Error>> {
        debug!("selecting rows");
        let rows: postgres::rows::Rows = self.conn
            .query(format!("SELECT k, v FROM {}", table).as_str(), &[])?;
        debug!("selected rows {:?}", rows.len());
        let mut res: Vec<(String, String)> = vec![];
        for row in rows.iter() {
            let row: postgres::rows::Row = row;
            let k: String = row.get(0);
            let v: String = row.get(1);
            res.push((k, v));
        }
        Ok(res)
    }

    pub fn set_kv_postgres(
        &self,
        table: &str,
        kvs: Vec<(String, String)>,
    ) -> Result<(), Box<Error>> {
        self.conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    k   VARCHAR PRIMARY KEY,
                    v   VARCHAR NOT NULL
                  )",
                table
            ).as_str(),
            &[],
        )?;
        debug!("inserting rows");
        let transaction = self.conn.transaction()?;
        for (k, v) in kvs {
            transaction.execute(
                format!("INSERT INTO {} (k, v) VALUES ($1, $2)", table).as_str(),
                &[&k, &v],
            )?;
        }
        transaction.commit()?;
        debug!("inserting rows - commmitted");
        Ok(())
    }
}
