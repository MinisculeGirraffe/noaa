use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

pub struct CO1 {
    climate_division: Option<i32>,
    time_conversion: RecordValue<i32>,
}
// co2-co9
pub struct COX {
    element_id: Option<String>,
    time_offset: RecordValue<i32>,
}

pub struct CR1 {
    version: RecordValue<i32>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}

//CT1-3
pub struct CTX {
    air_temp: RecordValue<i32>,
    air_temp_quality_code: CodeRecord,
    air_temp_quality_flag: CodeRecord,
}

//CU1-3
pub struct CUX {
    air_temp: RecordValue<i32>,
    air_temp_quality_code: CodeRecord,
    air_temp_quality_flag: CodeRecord,
    air_temp_std_dev: RecordValue<i32>,
    air_temp_std_dev_quality_code: CodeRecord,
    air_temp_std_dev_quality_flag: CodeRecord,
}
// CV1-3
pub struct CVX {
    air_temp_min: RecordValue<i32>,
    air_temp_min_quality_code: CodeRecord,
    air_temp_min_quality_flag: CodeRecord,
    air_temp_min_time: RecordValue<i32>,
    air_temp_min_time_quality_code: CodeRecord,
    air_temp_min_time_quality_flag: CodeRecord,
    air_temp_max: RecordValue<i32>,
    air_temp_max_quality_code: CodeRecord,
    air_temp_max_quality_flag: CodeRecord,
    air_temp_max_time: RecordValue<i32>,
    air_temp_max_time_quality_code: CodeRecord,
    air_temp_max_time_quality_flag: CodeRecord,
}

pub struct CW1 {
    wetness_1: RecordValue<i32>,
    wetness_1_quality_code: CodeRecord,
    wetness_1_quality_flag: CodeRecord,
    wetness_2: RecordValue<i32>,
    wetness_2_quality_code: CodeRecord,
    wetness_2_quality_flag: CodeRecord,
}

// CX1-3
pub struct CXX {
    precipitation: RecordValue<i32>,
    precipitation_quality_code: CodeRecord,
    precipitation_quality_flag: CodeRecord,
    avg_frequency: RecordValue<i32>,
    avg_frequency_quality_code: CodeRecord,
    avg_frequency_quality_flag: CodeRecord,
    min_frequency: RecordValue<i32>,
    min_frequency_quality_code: CodeRecord,
    min_frequency_quality_flag: CodeRecord,
    max_frequency: RecordValue<i32>,
    max_frequency_quality_code: CodeRecord,
    max_frequency_quality_flag: CodeRecord,
}
