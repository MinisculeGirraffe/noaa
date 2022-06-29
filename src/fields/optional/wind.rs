use crate::fields::codes::{CodeRecord, DL_QUALITY_CODES, DL_QUALITY_FLAG, QUALITY_CODES,DERIVE_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

pub static TYPE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
 "1" => "Average speed of prevailing wind",
 "2" => "Mean wind speed",
 "3" => "Maximum instantaneous wind speed",
 "4" => "Maximum gust speed",
 "5" => "Maximum mean wind speed",
 "6" => "Maximum 1-minute mean wind speed",
 "9" => "Missing",
};

pub static SUMMARY_TYPE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Peak wind speed for the day",
    "2" => "Fastest 2-minute wind speed for the day",
    "3" => "Average wind speed for the day",
    "4" => "Fastest 5-minute wind speed for the day",
    "5" => "Fastest mile wind speed for the day",
};

pub static RH_CODES: phf::Map<&'static str, &'static str> = phf_map! {
 "M" => "Mean relative humidity",
 "N" => "Minimum relative humidity",
 "X" => "Maximum relative humidity",
 "9" => "missing",
};
//oa1-oa5
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct OAX {
    type_code: CodeRecord,
    period_quantity: Option<RecordValue<f64>>,
    speed_rate: Option<RecordValue<f64>>,
    speed_quality_code: CodeRecord,
}
impl FromStr for OAX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(OAX {
            type_code: CodeRecord::new(&parts[0], &TYPE_CODES),
            period_quantity: RecordValue::<f64>::new(&parts[1], "hours", 1f64),
            speed_rate: RecordValue::<f64>::new(&parts[2], "m/s", 10f64),
            speed_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
//OB1-2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct OBX {
    wind_avg_time: Option<RecordValue<f64>>,
    wind_max_gust: Option<RecordValue<f64>>,
    wind_max_quality_code: CodeRecord,
    wind_max_quality_flag: CodeRecord,
    wind_max_direction: Option<RecordValue<f64>>,
    wind_max_direction_quality_code: CodeRecord,
    wind_max_direction_quality_flag: CodeRecord,
    wind_speed_std_dev: Option<RecordValue<f64>>,
    wind_speed_std_dev_quality_code: CodeRecord,
    wind_speed_std_dev_quality_flag: CodeRecord,
    wind_direction_std_dev: Option<RecordValue<f64>>,
    wind_direction_std_dev_quality_code: CodeRecord,
    wind_direction_std_dev_quality_flag: CodeRecord,
}

impl FromStr for OBX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(OBX {
            wind_avg_time: RecordValue::<f64>::new(&parts[0], "minutes", 1f64),
            wind_max_gust: RecordValue::<f64>::new(&parts[1], "m/s", 10f64),
            wind_max_quality_code: CodeRecord::new(&parts[2], &DL_QUALITY_CODES),
            wind_max_quality_flag: CodeRecord::new(&parts[3], &DL_QUALITY_FLAG),
            wind_max_direction: RecordValue::<f64>::new(&parts[4], "degrees", 1f64),
            wind_max_direction_quality_code: CodeRecord::new(&parts[5], &DL_QUALITY_CODES),
            wind_max_direction_quality_flag: CodeRecord::new(&parts[6], &DL_QUALITY_FLAG),
            wind_speed_std_dev: RecordValue::<f64>::new(&parts[7], "m/s", 100f64),
            wind_speed_std_dev_quality_code: CodeRecord::new(&parts[8], &DL_QUALITY_CODES),
            wind_speed_std_dev_quality_flag: CodeRecord::new(&parts[9], &DL_QUALITY_FLAG),
            wind_direction_std_dev: RecordValue::<f64>::new(&parts[10], "degrees", 100f64),
            wind_direction_std_dev_quality_code: CodeRecord::new(&parts[11], &DL_QUALITY_CODES),
            wind_direction_std_dev_quality_flag: CodeRecord::new(&parts[12], &DL_QUALITY_FLAG),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct OC1 {
    speed_rate: Option<RecordValue<f64>>,
    speed_quality_code: CodeRecord,
}
impl FromStr for OC1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(OC1 {
            speed_rate: RecordValue::<f64>::new(&parts[0], "m/s", 10f64),
            speed_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
        })
    }
}

//od1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct ODX {
    type_code: CodeRecord,
    period_quantity: Option<RecordValue<f64>>,
    direction_quantity: Option<RecordValue<f64>>,
    speed_rate: Option<RecordValue<f64>>,
    speed_quality_code: CodeRecord,
}
// TODO Revisit, order of direction/speed is unclear in documentation
impl FromStr for ODX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(ODX {
            type_code: CodeRecord::new(&parts[0], &TYPE_CODES),
            period_quantity: RecordValue::<f64>::new(&parts[1], "hours", 1f64),
            direction_quantity: RecordValue::<f64>::new(&parts[2], "degrees", 1f64),
            speed_rate: RecordValue::<f64>::new(&parts[3], "m/s", 10f64),
            speed_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}

//OE1-OE3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct OEX {
    type_code: CodeRecord,
    period_quantity: Option<RecordValue<f64>>,
    speed_rate: Option<RecordValue<f64>>,
    direction: Option<RecordValue<f64>>,
    time: Option<RecordValue<f64>>,
    quality_code: CodeRecord,
}
impl FromStr for OEX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(OEX {
            type_code: CodeRecord::new(&parts[0], &SUMMARY_TYPE_CODES),
            period_quantity: RecordValue::<f64>::new(&parts[1], "hours", 1f64),
            speed_rate: RecordValue::<f64>::new(&parts[2], "m/s", 100f64),
            direction: RecordValue::<f64>::new(&parts[3], "degrees", 1f64),
            time: RecordValue::<f64>::new(&parts[4], "hh:mm", 1f64),
            quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

//RH1-RH3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct RHX {
    period_quantity: Option<RecordValue<f64>>,
    code: CodeRecord,
    percentage: Option<RecordValue<f64>>,
    derrived_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for RHX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(RHX {
            period_quantity: RecordValue::<f64>::new(&parts[0], "hours", 1f64),
            code: CodeRecord::new(&parts[1], &RH_CODES),
            percentage: RecordValue::<f64>::new(&parts[2], "%", 1f64),
            derrived_code: CodeRecord::new(&parts[3], &DERIVE_CODES),
            quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}