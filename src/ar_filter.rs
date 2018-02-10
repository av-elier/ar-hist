use serde_json;
use std::error::Error;
use std::collections::BTreeSet;

pub fn filter_spahshots(orig: Vec<(&str, &str)>) -> Result<Vec<(String, String)>, Box<Error>> {
    // TODO: rewrite as stream to handle unlimited number of values
    let mut filt: Vec<(String, String)> = Vec::new();
    let mut all: Vec<serde_json::Value> = vec![];
    orig.iter().try_fold(
        (),
        |_: (), &(k, v): &(&str, &str)| -> Result<(), Box<Error>> {
            let mut latest: Vec<serde_json::Value> = serde_json::from_str(v)?;
            merge(&mut all, &mut latest);
            let test = latest.clone();
            let v_filt = serde_json::to_string(&test)?;
            filt.push((k.to_string(), v_filt));
            Ok(())
        },
    )?;
    Ok(filt)
}

pub fn merge(all: &mut Vec<serde_json::Value>, latest: &mut Vec<serde_json::Value>) {
    if all.is_empty() {
        all.append(&mut latest.clone());
        return;
    }

    let mut replace_in_all: Vec<serde_json::Value> = vec![];
    let mut replace_in_all_ids: BTreeSet<i64> = BTreeSet::new();
    let mut add_to_all: Vec<serde_json::Value> = vec![];
    let mut remove_in_latest_ids: BTreeSet<i64> = BTreeSet::new();

    for lv in latest.iter() {
        let mut found_id = false;
        let mut found_value = false;
        for av in all.iter() {
            if lv["id"] == av["id"] {
                found_id = true;
                if lv == av {
                    found_value = true;
                }
                break;
            }
        }
        match (found_id, found_value) {
            (true, true) => {
                remove_in_latest_ids.insert(lv["id"].as_i64().unwrap());
            }
            (true, false) => {
                replace_in_all.push(lv.clone());
                replace_in_all_ids.insert(lv["id"].as_i64().unwrap());
            }
            (false, _) => {
                add_to_all.push(lv.clone());
            }
        }
    }

    all.retain(|x| !replace_in_all_ids.contains(&x["id"].as_i64().unwrap()));
    all.append(&mut replace_in_all);
    all.append(&mut add_to_all);

    latest.retain(|x| !remove_in_latest_ids.contains(&x["id"].as_i64().unwrap()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn not_filter_when_nothing_to() {
        let orig = || vec![("a", r#"[{"a":"b","id":4}]"#)];
        let filt = super::filter_spahshots(orig()).unwrap();
        assert_eq!(
            filt,
            orig()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<(String, String)>>()
        );
    }
    #[test]
    fn filter_identical() {
        let orig = {
            vec![
                ("a", r#"[{"a":"b","id":4}]"#),
                ("b", r#"[{"a":"b","id":4}]"#),
                ("c", r#"[{"a":"b","id":4}, {"e":{"r":321},"id":11}]"#),
                ("d", r#"[{"a":"q","id":4}, {"e":{"r":321},"id":11}]"#),
            ]
        };
        let expect = || {
            vec![
                ("a", r#"[{"a":"b","id":4}]"#),
                ("b", r#"[]"#),
                ("c", r#"[{"e":{"r":321},"id":11}]"#),
                ("d", r#"[{"a":"q","id":4}]"#),
            ]
        };
        let filt = super::filter_spahshots(orig).unwrap();
        assert_eq!(
            filt,
            expect()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<(String, String)>>()
        );
    }
}
