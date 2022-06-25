use crate::fields::mandatory::{Ceiling, SeaLevelPressure, Temprature, Visibility, Wind};
use crate::fields::optional::cloud_solar::*;
use crate::fields::optional::network_metadata::*;
use crate::fields::optional::precipitation::*;
use crate::fields::optional::weather_occourance::*;
use crate::fields::optional::climate_reference_network::*;
use crate::fields::optional::runway_visual_range::*;
use crate::util::*;
use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

#[skip_serializing_none]
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
    aa1: Option<AAX>,
    aa2: Option<AAX>,
    aa3: Option<AAX>,
    aa4: Option<AAX>,
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
    mw1: Option<MWX>,
    mw2: Option<MWX>,
    mw3: Option<MWX>,
    mw4: Option<MWX>,
    mw5: Option<MWX>,
    mw6: Option<MWX>,
    mw7: Option<MWX>,
    //climate_reference_network,
    cb1: Option<CBX>,
    cb2: Option<CBX>,
    cf1: Option<CFX>,
    cf2: Option<CFX>,
    cf3: Option<CFX>,
    cg1: Option<CGX>,
    cg2: Option<CGX>,
    cg3: Option<CGX>,
    ch1: Option<CHX>,
    ch2: Option<CHX>,
    ci1: Option<CI1>,
    cn1: Option<CN1>,
    cn2: Option<CN2>,
    cn3: Option<CN3>,
    cn4: Option<CN4>,
    //network_metadata
    co1: Option<CO1>,
    co2: Option<COX>,
    co3: Option<COX>,
    co4: Option<COX>,
    co5: Option<COX>,
    co6: Option<COX>,
    co7: Option<COX>,
    co8: Option<COX>,
    co9: Option<COX>,
    cr1: Option<CR1>,
    ct1: Option<CTX>,
    ct2: Option<CTX>,
    ct3: Option<CTX>,
    cu1: Option<CUX>,
    cu2: Option<CUX>,
    cu3: Option<CUX>,
    cv1: Option<CVX>,
    cv2: Option<CVX>,
    cv3: Option<CVX>,
    cw1: Option<CW1>,
    cx1: Option<CXX>,
    cx2: Option<CXX>,
    cx3: Option<CXX>,
    //runway_visual_range
    ed1: Option<ED1>,
    //cloud_solar:
    ga1: Option<GAX>,
    ga2: Option<GAX>,
    ga3: Option<GAX>,
    ga4: Option<GAX>,
    ga5: Option<GAX>,
    ga6: Option<GAX>,
    gd1: Option<GDX>,
    gd2: Option<GDX>,
    gd3: Option<GDX>,
    gd4: Option<GDX>,
    gd5: Option<GDX>,
    gd6: Option<GDX>,
    ge1: Option<GE1>,
    gf1: Option<GF1>,
    gg1: Option<GGX>,
    gg2: Option<GGX>,
    gg3: Option<GGX>,
    gg4: Option<GGX>,
    gg5: Option<GGX>,
    gg6: Option<GGX>,
    gh1: Option<GH1>,
    gj1: Option<GJ1>,
    gk1: Option<GK1>,
    gl1: Option<GL1>,
    gm1: Option<GM1>,
    gn1: Option<GN1>,
    go1: Option<GO1>,
    gp1: Option<GP1>,
    gq1: Option<GQ1>,
    gr1: Option<GR1>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RecordValue<T> {
    value: T,
    unit: String,
}
impl<T> RecordValue<T> {
    pub fn new(s: &str, unit: &str, divide: T) -> Option<RecordValue<T>>
    where
        T: FromStr + std::ops::Div<Output = T>,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut check = false;

        //Check if the value isn't all 9's
        for c in s.chars() {
            if c.is_numeric() && c != '9' {
                check = true;
                break;
            }
        }

        if check {
            Some(RecordValue {
                value: T::from_str(s).unwrap() / divide,
                unit: unit.to_string(),
            })
        } else {
            None
        }
    }
}
