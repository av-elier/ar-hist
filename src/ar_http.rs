use std;
use std::error::Error;
use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json;
use serde_json::Value;

pub fn do_http(page: i32/* , order: i32, aasm_state: String */) -> Result<Vec<Value>, Box<Error>> {
    let order = 1;
    let arurl = format!("http://ar.rostov-gorod.ru/initiatives.json?filter%5Binitiative_from%5D=&order={0}&page={1}",
        order, page);

    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = arurl.parse()?;
    let work = client.get(uri).and_then(|res| {
        debug!("page = {}, response = {}", page, res.status());

        res.body().concat2()
    }).and_then(move |body| {
        debug!("body head = {}", std::string::String::from_utf8(body.to_vec())?.chars().take(10).collect::<String>());
        let v: Vec<Value> =
            serde_json::from_slice(&body).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(v)
    });

    Ok(core.run(work)?)
}

pub fn get_ar_json_vec() -> Result<Vec<Value>, Box<Error>> {
    let mut res: Vec<Value> = Vec::new();
    for i in 1..100 {
        for _ in 1..5 {
            let mut values = do_http(i);
            if let Err(_) = values {
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

#[derive(Serialize, Deserialize, Debug)]
struct ArStruct {
    // "id": 1067,
    id: i32,
    // "user_id": 15590,
    user_id: i32,
    // "status": "Активна",
    status: String,

    // "poll_attributes": null,
    // "created_at": "2017-12-25T13:10:14.295+03:00",
    // "published_at": "2017-12-27T15:21:51.590+03:00",
    // "aasm_state": "active",
    // "considered": null,
    // "attached": null,
    // "attach_me": null,
    // "user": {
    //   "fullname": "Дмитрий Бородин",
    //   "id": 15590
    // },
    // "title": "Светофор на перекрестке пер. Днепровский и ул. Каскадная",
    // "category_id": 9,
    // "initiative_type": "voting",
    // "positive": 35,
    // "negative": 12,
    // "category": {
    //   "id": 9,
    //   "title": "Транспорт и дороги"
    // },
    // "description": "Уважаемые организаторы дорожного движения в г. Ростов-на-Дону. Прошу вас перевести светофор на перекрестке пер. Днепровский и ул. Каскадная в режим желтый мигающий сигнал, т. к. после того, как ввели в работу светофор, стали организовываться огромные пробки по всем улицам и переулкам, которые примыкают к данному перекрестку. До установления светофора, проехать данный перекресток можно было без затруднений и потерь времени.",
    // "expire_date": "2018-03-25T00:00:00.000+03:00",
    // "address": null,
    // "lat": null,
    // "lng": null,
    // "images": [],
    // "documents": [],
    // "comments_count": 2,
    // "watch_count": 258,
    // "accompanying_attributes": null,
    // "routes": {
    //   "share": {
    //     "vk": "/shares/share?id=1067&klass=Initiative&type=vk",
    //     "ok": "/shares/share?id=1067&klass=Initiative&type=ok",
    //     "fb": "/shares/share?id=1067&klass=Initiative&type=fb",
    //     "tw": "/shares/share?id=1067&klass=Initiative&type=tw"
    //   }
    // },
    // "statistic": {
    //   "shares": {
    //     "ok": 1,
    //     "vk": 1,
    //     "total": 2
    //   }
    // },
    // "department": null
}
