use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

pub static RANGE_OBSERVATION: phf::Map<&'static str, &'static str> = phf_map! {
    "L" => "left",
    "C" => "center",
    "R" => "right",
    "U" => "unknown",
    "9" => "missing",
};

pub struct ED1 {
    direction_angle: RecordValue<i32>,
    designator_code: CodeRecord,
    visibility_dimension: RecordValue<i32>,
    quality_code: CodeRecord,
}