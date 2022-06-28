use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct SA1 {
    temperature: Option<RecordValue<f64>>,
    temperature_quality_code: CodeRecord,
}
impl FromStr for SA1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(SA1 {
            temperature: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            temperature_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
