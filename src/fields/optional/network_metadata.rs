use crate::fields::codes::{CodeRecord, DL_QUALITY_CODES, DL_QUALITY_FLAG};
use crate::model::RecordValue;
use crate::util::{get_parts, is_null, parse_str};
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CO1 {
    climate_division: Option<RecordValue<i32>>,
    time_conversion: Option<RecordValue<i32>>,
}
impl FromStr for CO1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CO1 {
            climate_division: RecordValue::<i32>::new(&parts[0], "", 1),
            time_conversion: RecordValue::<i32>::new(&parts[1], "h", 1),
        })
    }
}

// cO2-cO9
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct COX {
    element_id: Option<String>,
    time_offset: Option<RecordValue<f64>>,
}
impl FromStr for COX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        //todo revisit later. This is wonky and might break things
        let element_id = match is_null(&parts[0]) {
            true => None,
            false => Some(parts[0].to_string()),
        };
        Ok(COX {
            element_id,
            time_offset: RecordValue::<f64>::new(&parts[1], "h", 10f64),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CR1 {
    version: Option<RecordValue<f64>>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}
impl FromStr for CR1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CR1 {
            version: RecordValue::<f64>::new(&parts[0], "", 1000f64),
            quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
        })
    }
}

//CT1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CTX {
    air_temp: Option<RecordValue<f64>>,
    air_temp_quality_code: CodeRecord,
    air_temp_quality_flag: CodeRecord,
}
impl FromStr for CTX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CTX {
            air_temp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            air_temp_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            air_temp_quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
        })
    }
}
//CT1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//CU1-3
pub struct CUX {
    air_temp: Option<RecordValue<f64>>,
    air_temp_quality_code: CodeRecord,
    air_temp_quality_flag: CodeRecord,
    air_temp_std_dev: Option<RecordValue<f64>>,
    air_temp_std_dev_quality_code: CodeRecord,
    air_temp_std_dev_quality_flag: CodeRecord,
}

impl FromStr for CUX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CUX {
            air_temp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            air_temp_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            air_temp_quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
            air_temp_std_dev: RecordValue::<f64>::new(&parts[3], "°C", 10f64),
            air_temp_std_dev_quality_code: CodeRecord::new(&parts[4], &DL_QUALITY_CODES),
            air_temp_std_dev_quality_flag: CodeRecord::new(&parts[5], &DL_QUALITY_FLAG),
        })
    }
}

// CV1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CVX {
    air_temp_min: Option<RecordValue<f64>>,
    air_temp_min_quality_code: CodeRecord,
    air_temp_min_quality_flag: CodeRecord,
    air_temp_min_time: Option<String>,
    air_temp_min_time_quality_code: CodeRecord,
    air_temp_min_time_quality_flag: CodeRecord,
    air_temp_max: Option<RecordValue<f64>>,
    air_temp_max_quality_code: CodeRecord,
    air_temp_max_quality_flag: CodeRecord,
    air_temp_max_time: Option<String>,
    air_temp_max_time_quality_code: CodeRecord,
    air_temp_max_time_quality_flag: CodeRecord,
}

impl FromStr for CVX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CVX {
            air_temp_min: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            air_temp_min_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            air_temp_min_quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
            air_temp_min_time: parse_str(&parts[3]),
            air_temp_min_time_quality_code: CodeRecord::new(&parts[4], &DL_QUALITY_CODES),
            air_temp_min_time_quality_flag: CodeRecord::new(&parts[5], &DL_QUALITY_FLAG),
            air_temp_max: RecordValue::<f64>::new(&parts[6], "°C", 10f64),
            air_temp_max_quality_code: CodeRecord::new(&parts[7], &DL_QUALITY_CODES),
            air_temp_max_quality_flag: CodeRecord::new(&parts[8], &DL_QUALITY_FLAG),
            air_temp_max_time: parse_str(&parts[9]),
            air_temp_max_time_quality_code: CodeRecord::new(&parts[10], &DL_QUALITY_CODES),
            air_temp_max_time_quality_flag: CodeRecord::new(&parts[11], &DL_QUALITY_FLAG),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CW1 {
    wetness_1: Option<RecordValue<f64>>,
    wetness_1_quality_code: CodeRecord,
    wetness_1_quality_flag: CodeRecord,
    wetness_2: Option<RecordValue<f64>>,
    wetness_2_quality_code: CodeRecord,
    wetness_2_quality_flag: CodeRecord,
}
impl FromStr for CW1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CW1 {
            wetness_1: RecordValue::<f64>::new(&parts[0], "", 10f64),
            wetness_1_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            wetness_1_quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
            wetness_2: RecordValue::<f64>::new(&parts[3], "", 10f64),
            wetness_2_quality_code: CodeRecord::new(&parts[4], &DL_QUALITY_CODES),
            wetness_2_quality_flag: CodeRecord::new(&parts[5], &DL_QUALITY_FLAG),
        })
    }
}

// CX1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CXX {
    precipitation_total_hourly: Option<RecordValue<f64>>,
    precipitation_total_hourly_quality_code: CodeRecord,
    precipitation_total_hourly_quality_flag: CodeRecord,
    avg_frequency: Option<RecordValue<f64>>,
    avg_frequency_quality_code: CodeRecord,
    avg_frequency_quality_flag: CodeRecord,
    min_frequency: Option<RecordValue<f64>>,
    min_frequency_quality_code: CodeRecord,
    min_frequency_quality_flag: CodeRecord,
    max_frequency: Option<RecordValue<f64>>,
    max_frequency_quality_code: CodeRecord,
    max_frequency_quality_flag: CodeRecord,
}

impl FromStr for CXX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(CXX {
            precipitation_total_hourly: RecordValue::<f64>::new(&parts[0], "mm", 10f64),
            precipitation_total_hourly_quality_code: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            precipitation_total_hourly_quality_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
            avg_frequency: RecordValue::<f64>::new(&parts[3], "Hz", 10f64),
            avg_frequency_quality_code: CodeRecord::new(&parts[4], &DL_QUALITY_CODES),
            avg_frequency_quality_flag: CodeRecord::new(&parts[5], &DL_QUALITY_FLAG),
            min_frequency: RecordValue::<f64>::new(&parts[6], "Hz", 10f64),
            min_frequency_quality_code: CodeRecord::new(&parts[7], &DL_QUALITY_CODES),
            min_frequency_quality_flag: CodeRecord::new(&parts[8], &DL_QUALITY_FLAG),
            max_frequency: RecordValue::<f64>::new(&parts[9], "Hz", 10f64),
            max_frequency_quality_code: CodeRecord::new(&parts[10], &DL_QUALITY_CODES),
            max_frequency_quality_flag: CodeRecord::new(&parts[11], &DL_QUALITY_FLAG),
        })
    }
}
