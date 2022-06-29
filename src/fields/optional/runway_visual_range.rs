use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

pub static RANGE_OBSERVATION: phf::Map<&'static str, &'static str> = phf_map! {
    "L" => "left",
    "C" => "center",
    "R" => "right",
    "U" => "unknown",
    "9" => "missing",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct ED1 {
    direction_angle: Option<RecordValue<f64>>,
    designator_code: CodeRecord,
    visibility_dimension: Option<RecordValue<f64>>,
    quality_code: CodeRecord,
}
impl FromStr for ED1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(ED1 {
            direction_angle: RecordValue::<f64>::new(&parts[0], "°", 0.1f64),
            designator_code: CodeRecord::new(&parts[1], &RANGE_OBSERVATION),
            visibility_dimension: RecordValue::<f64>::new(&parts[2], "m", 1f64),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
