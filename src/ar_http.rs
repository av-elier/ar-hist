use std;
use std::error::Error;
use std::io;
use futures::{Future, Stream};
use hyper::{Chunk, Client, Error as HError};
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use serde;
use serde_json;

fn ar_http_future<'a, T>(
    client: &Client<HttpConnector>,
    page: i32,
    status: Option<&str>,
) -> Result<Box<Future<Item = Vec<T>, Error = HError>>, Box<Error>>
where
    for<'de> T: serde::Deserialize<'de> + 'static,
{
    let order = 1;
    let aasm_state = match status {
        Some(status) => format!("&filter%5Baasm_state%5D={}", status),
        None => "".to_string(),
    };
    let arurl = format!("http://ar.rostov-gorod.ru/initiatives.json?filter%5Binitiative_from%5D=&order={0}&page={1}{2}",
        order, page, aasm_state);
    let uri = arurl.parse()?;

    let work = Box::new(
        client
            .get(uri)
            .and_then(move |res| {
                debug!("page = {}, response = {}", page, res.status());

                res.body().concat2()
            })
            .and_then(move |body: Chunk| {
                let ar_json: &str = std::str::from_utf8(&body)?;
                debug!(
                    "body head = {}",
                    ar_json.to_string().chars().take(10).collect::<String>()
                );
                let tpd: Vec<T> = serde_json::from_slice(&body)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok(tpd)
            }),
    );
    Ok(Box::new(work))
}

fn do_ar_work<'a, T>(
    core: &mut Core,
    client: &Client<HttpConnector>,
    page: i32,
    status: Option<&str>,
) -> Result<Vec<T>, Box<Error>>
where
    for<'de> T: serde::Deserialize<'de> + 'static,
{
    let work = ar_http_future::<T>(&client, page, status)?;
    let body: Vec<T> = core.run(work)?;
    Ok(body)
}

pub fn get_ar_json_vec<T>(status: Option<&str>) -> Result<Vec<T>, Box<Error>>
where
    for<'de> T: serde::Deserialize<'de> + 'static,
{
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let mut res: Vec<T> = Vec::new();
    for i in 1..100 {
        for try in 1..5 {
            let values = do_ar_work::<T>(&mut core, &client, i, status);
            if let Err(err) = values {
                error!("error in downloading, try={}, err={:?}", try, err);
                continue;
            } else if let Ok(mut values) = values {
                if values.len() == 0 {
                    return Ok(res);
                }
                res.append(&mut values);
                break;
            }
        }
    }
    Ok(res)
}
