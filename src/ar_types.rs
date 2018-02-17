use serde_json;
use std::error::Error;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Initiative {
    // "id": 1067,
    pub id: i32,
    // "user_id": 15590,
    pub user_id: i32,
    // "status": "Активна",
    pub status: String, // TODO: enum
    // "aasm_state": "active",
    pub aasm_state: String, // TODO: enum
    // "poll_attributes": null,
    // "created_at": "2017-12-25T13:10:14.295+03:00",
    pub created_at: String, // TODO: DateTime<Utc>
    // "published_at": "2017-12-27T15:21:51.590+03:00",
    pub published_at: String, // TODO: DateTime<Utc>
    // "considered": null,
    // "attached": null,
    // "attach_me": null,
    // "user": {
    //   "fullname": "Дмитрий Бородин",
    //   "id": 15590
    // },
    pub user: User,
    // "title": "Светофор на перекрестке пер. Днепровский и ул. Каскадная",
    pub title: String,
    // "category_id": 9,
    pub category_id: i32,
    // "initiative_type": "voting",
    pub initiative_type: String, // TODO: enum

    // "positive": 35,
    pub positive: i32,
    // "negative": 12,
    pub negative: i32,
    // "category": {
    //   "id": 9,
    //   "title": "Транспорт и дороги"
    // },
    pub category: Category,
    // "description": "Уважаемые организаторы дорожного движения в г. Ростов-на-Дону. Прошу вас перевести светофор на перекрестке пер. Днепровский и ул. Каскадная в режим желтый мигающий сигнал, т. к. после того, как ввели в работу светофор, стали организовываться огромные пробки по всем улицам и переулкам, которые примыкают к данному перекрестку. До установления светофора, проехать данный перекресток можно было без затруднений и потерь времени.",
    pub description: String,
    // "expire_date": "2018-03-25T00:00:00.000+03:00",
    pub expire_date: String, // TODO: DateTime<Utc>
    // "address": null,
    // "lat": null,
    // "lng": null,
    // "images": [],
    // "documents": [],
    // "comments_count": 2,
    pub comments_count: i32,
    // "watch_count": 258,
    pub watch_count: i32,
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
    pub statistic: ShareStatistic,
    // "department": null
}

impl Initiative {
    pub fn lifetime_percent(&self) -> f64 {
        let lt = self.try_lifetime_percent();
        match lt {
            Ok(x) => x,
            Err(_) => 100.0,
        }
    }
    fn try_lifetime_percent(&self) -> Result<f64, Box<Error>> {
        let a = DateTime::parse_from_rfc3339(&self.created_at)?
            .with_timezone(&Utc)
            .timestamp() as f64;
        let b = DateTime::parse_from_rfc3339(&self.expire_date)?
            .with_timezone(&Utc)
            .timestamp() as f64;
        let x = Utc::now().timestamp() as f64;
        let percent = (x - a) / (b - a) * 100.0;
        let trunc_percent = if percent < 0.0 {
            0.0
        } else if percent > 100.0 {
            100.0
        } else {
            percent
        };
        Ok(trunc_percent)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    // "id": 15590
    pub id: i32,
    // "fullname": "Дмитрий Бородин",
    pub fullname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    // "id": 9,
    pub id: i32,
    // "title": "Транспорт и дороги"
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareStatistic(serde_json::Value);
impl ShareStatistic {
    pub fn sum(&self) -> i32 {
        match &self.0.get("shares") {
            &Some(obj) => match obj {
                &serde_json::Value::Object(ref map) => {
                    let mut sum = 0i32;
                    for v in map.values() {
                        if let Some(v) = v.as_i64() {
                            sum += v as i32;
                        }
                    }
                    sum
                }
                _ => -1,
            },
            _ => -2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShareStatistic;
    #[test]
    fn share_stat_sum() {
        let ss = ShareStatistic(json!({
           "shares": {
             "ok": 1,
             "vk": 1,
             "total": 2
          }
        }));
        assert_eq!(4, ss.sum());
    }
}
