use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::iter::repeat;
use std::str::FromStr;

use crate::fields::mandatory::{Ceiling, SeaLevelPressure, Temprature, Visibility, Wind};
use crate::fields::optional::precipitation::*;
use crate::fields::optional::weather_occourance::*;
use crate::util::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
pub struct Record {
    station: String,
    #[serde(serialize_with = "str_from_native_date_time")]
    #[serde(deserialize_with = "naive_date_time_from_str")]
    date: NaiveDateTime,
    source: String,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    name: String,
    report_type: String,
    #[serde(deserialize_with = "remove_whitespace")]
    call_sign: String,
    quality_control: String,
    wnd: Wind,
    cig: Ceiling,
    vis: Visibility,
    tmp: Temprature,
    slp: SeaLevelPressure,
    // Precipitation
    aa1: Option<AAx>,
    aa2: Option<AAx>,
    aa3: Option<AAx>,
    aa4: Option<AAx>,
    ab1: Option<AB1>,
    ac1: Option<AC1>,
    ad1: Option<AD1>,
    ae1: Option<AE1>,
    ag1: Option<AG1>,
    ah1: Option<AHX>,
    ah2: Option<AHX>,
    ah3: Option<AHX>,
    ah4: Option<AHX>,
    ah5: Option<AHX>,
    ah6: Option<AHX>,
    ai1: Option<AIX>,
    ai2: Option<AIX>,
    ai3: Option<AIX>,
    ai4: Option<AIX>,
    ai5: Option<AIX>,
    ai6: Option<AIX>,
    aj1: Option<AJ1>,
    ak1: Option<AK1>,
    al1: Option<ALX>,
    al2: Option<ALX>,
    al3: Option<ALX>,
    al4: Option<ALX>,
    am1: Option<AM1>,
    an1: Option<AN1>,
    ao1: Option<AOX>,
    ao2: Option<AOX>,
    ao3: Option<AOX>,
    ao4: Option<AOX>,
    // Weather Occurrence Data
    at1: Option<ATX>,
    at2: Option<ATX>,
    at3: Option<ATX>,
    at4: Option<ATX>,
    at5: Option<ATX>,
    at6: Option<ATX>,
    at7: Option<ATX>,
    at8: Option<ATX>,
    au1: Option<AUX>,
    au2: Option<AUX>,
    au3: Option<AUX>,
    au4: Option<AUX>,
    au5: Option<AUX>,
    au6: Option<AUX>,
    au7: Option<AUX>,
    au8: Option<AUX>,
    au9: Option<AUX>,
    aw1: Option<AWX>,
    aw2: Option<AWX>,
    aw3: Option<AWX>,
    aw4: Option<AWX>,
    ax1: Option<AXX>,
    ax2: Option<AXX>,
    ax3: Option<AXX>,
    ax4: Option<AXX>,
    ax5: Option<AXX>,
    ax6: Option<AXX>,
    ay1: Option<AYX>,
    ay2: Option<AYX>,
    az1: Option<AZX>,
    az2: Option<AZX>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RecordValue<T> {
    value: Option<T>,
    unit: String,
}
impl<T> RecordValue<T> {
    pub fn new(s: &str, unit: &str, divide: T) -> RecordValue<T>
    where
        T: FromStr + std::ops::Div<Output = T>,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut val = None;
        let mut check = false;

        //Check if the value isn't all 9's
        for c in s.chars() {
            if c.is_numeric() && c != '9' {
                check = true;
                // divide inner by divide and then convert to option
                val = Some(T::from_str(s).unwrap() / divide);
                break;
            }
        }
        RecordValue {
            value: val,
            unit: unit.to_string(),
        }
    }
}
