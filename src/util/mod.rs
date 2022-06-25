use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer,  Serializer};




pub fn get_parts(s: &str) -> Vec<String>{
    s.split(",").map(|s| s.to_string()).collect()
}

pub fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%T").map_err(de::Error::custom)
}

pub fn str_from_native_date_time<S>(x: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // convert x to string
    serializer.serialize_str(&x.format("%Y-%m-%dT%T").to_string())
}

pub fn remove_whitespace<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    s.retain(|c| !c.is_whitespace()); // remove all whitespace
    Ok(s)
}


pub fn is_null(s: &str) -> bool {
    for c in s.chars() {
        if c.is_numeric() && c != '9' {
            return false;
        }
    }
    true
}



pub fn parse_null (s: &str) -> Option<String> {
    if is_null(s) {
        None
    } else {
        Some(s.to_string())
    }
}
