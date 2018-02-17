use std::env;
use std::error::Error;
use postgres;
use openssl::ssl::{SslConnector, SslConnectorBuilder, SslMethod, SSL_VERIFY_NONE};
use ar_filter;
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
                format!("INSERT INTO {} (k, v) VALUES ($1, $2) ON CONFLICT (k) DO UPDATE SET v=EXCLUDED.v", table).as_str(),
                &[&k, &v],
            )?;
        }
        transaction.commit()?;
        debug!("inserting rows - commmitted");
        Ok(())
    }

    pub fn get_kv_merged(&self, table: &str) -> Result<Vec<(String, String)>, Box<Error>> {
        debug!("selecting rows");
        let rows: postgres::rows::Rows = self.conn
            .query(format!("SELECT k, v FROM {}", table).as_str(), &[])?;
        debug!("selected rows {:?}", rows.len());
        let mut res: Vec<(String, String)> = vec![];
        let mut all = String::from("[]");
        for row in rows.iter() {
            let row: postgres::rows::Row = row;
            let k: String = row.get(0);
            let mut v: String = row.get(1);
            all = ar_filter::merge_str(&mut all, &mut v)?;
            res.push((k, all.clone()));
        }
        Ok(res)
    }
}
