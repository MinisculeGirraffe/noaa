use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// CB1-2
pub struct CBX {
    period_quantity: RecordValue<u8>,
    liquid_depth: RecordValue<i32>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}

// CF1-3
pub struct CFX {
    rotations_per_second: RecordValue<i32>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}

pub struct CGX {
    liquid_depth: RecordValue<i32>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}

pub struct CHX {
    period_quantity: RecordValue<u8>,
    avg_air_temp: RecordValue<i32>,
    avg_air_temp_quality_code: CodeRecord,
    avg_air_temp_quality_flag: CodeRecord,
    avg_rh: RecordValue<i32>,
    avg_rh_quality_code: CodeRecord,
    avg_rh_quality_flag: CodeRecord,
}

pub struct CI1 {
    hourly_min_air_temp: RecordValue<i32>,
    hourly_min_air_temp_quality_code: CodeRecord,
    hourly_min_air_temp_quality_flag: CodeRecord,
    hourly_max_air_temp: RecordValue<i32>,
    hourly_max_air_temp_quality_code: CodeRecord,
    hourly_max_air_temp_quality_flag: CodeRecord,
    std_dev_air_temp: RecordValue<i32>,
    std_dev_air_temp_quality_code: CodeRecord,
    std_dev_air_temp_quality_flag: CodeRecord,
    std_dev_rh: RecordValue<i32>,
    std_dev_rh_quality_code: CodeRecord,
    std_dev_rh_quality_flag: CodeRecord,
}

pub struct CN1 {
    battery_voltage: RecordValue<i32>,
    battery_voltage_quality_code: CodeRecord,
    battery_voltage_quality_flag: CodeRecord,
    batter_voltage_full_load: RecordValue<i32>,
    battery_voltage_full_load_quality_code: CodeRecord,
    battery_voltage_full_load_quality_flag: CodeRecord,
    battery_voltage_data_logger: RecordValue<i32>,
    battery_voltage_data_logger_quality_code: CodeRecord,
    battery_voltage_data_logger_quality_flag: CodeRecord,
}
pub struct CN2 {
    tinlet_temp: RecordValue<i32>,
    tinlet_temp_quality_code: CodeRecord,
    tinlet_temp_quality_flag: CodeRecord,
    tinlet_max_temp: RecordValue<i32>,
    tinlet_max_temp_quality_code: CodeRecord,
    tinlet_max_temp_quality_flag: CodeRecord,
    door_open_time: RecordValue<i32>,
    door_open_time_quality_code: CodeRecord,
    door_open_time_quality_flag: CodeRecord,
}

pub struct CN3 {
    ref_res_avg: RecordValue<i32>,
    ref_res_avg_quality_code: CodeRecord,
    ref_res_avg_quality_flag: CodeRecord,
    d_signature: RecordValue<i32>,
    d_signature_quality_code: CodeRecord,
    d_signature_quality_flag: CodeRecord,
}

pub struct CN4 {
    gague_heater_flag: CodeRecord,
    gague_heater_quality_code: CodeRecord,
    gague_heater_quality_flag: CodeRecord,
    door_flag: CodeRecord,
    door_flag_quality_code: CodeRecord,
    door_flag_quality_flag: CodeRecord,
    fort_trans: RecordValue<i32>,
    fort_trans_quality_code: CodeRecord,
    refl_trans: RecordValue<i32>,
    refl_trans_quality_code: CodeRecord,
    refl_trans_quality_flag: CodeRecord,
}
