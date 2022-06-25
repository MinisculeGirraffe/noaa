use std::str::FromStr;

use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{Serialize};
use serde_with::DeserializeFromStr;

pub static QUALITY_FLAG: phf::Map<&'static str, &'static str> = phf_map! {
   "0" => "Passed all quality control checks",
   "1" => "Did not pass all quality check",
   "2" => "Did not pass all quality check",
   "3" => "Did not pass all quality check",
   "4" => "Did not pass all quality check",
   "5" => "Did not pass all quality check",
   "6" => "Did not pass all quality check",
   "7" => "Did not pass all quality check",
   "8" => "Did not pass all quality check",
   "9" => "Did not pass all quality check",
};
pub static GAGUE_HEATER_FLAG: phf::Map<&'static str, &'static str> = phf_map! {
 "0" => "Off",
 "1" => "On",
 "9" => "Missing",
};

// CB1-2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CBX {
    period_quantity: Option<RecordValue<isize>>,
    liquid_depth: Option<RecordValue<f64>>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}
impl FromStr for CBX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CBX {
            period_quantity: RecordValue::<isize>::new(&parts[0], "Minutes", 1),
            liquid_depth: RecordValue::<f64>::new(&parts[1], "mm", 10f64),
            quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            quality_flag: CodeRecord::new(&parts[3], &QUALITY_FLAG),
        })
    }
}

// CF1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CFX {
    fan_speed: Option<RecordValue<i32>>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}

impl FromStr for CFX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CFX {
            fan_speed: RecordValue::<i32>::new(&parts[0], "RPM", 1),
            quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
        })
    }
}
//CG1-3
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CGX {
    liquid_depth: Option<RecordValue<f64>>,
    quality_code: CodeRecord,
    quality_flag: CodeRecord,
}
impl FromStr for CGX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CGX {
            liquid_depth: RecordValue::<f64>::new(&parts[0], "mm", 10f64),
            quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
        })
    }
}
//CH1-2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CHX {
    period_quantity: Option<RecordValue<u8>>,
    avg_air_temp: Option<RecordValue<f64>>,
    avg_air_temp_quality_code: CodeRecord,
    avg_air_temp_quality_flag: CodeRecord,
    avg_rh: Option<RecordValue<f64>>,
    avg_rh_quality_code: CodeRecord,
    avg_rh_quality_flag: CodeRecord,
}
impl FromStr for CHX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CHX {
            period_quantity: RecordValue::<u8>::new(&parts[0], "Minutes", 1),
            avg_air_temp: RecordValue::<f64>::new(&parts[1], "°C", 10f64),
            avg_air_temp_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            avg_air_temp_quality_flag: CodeRecord::new(&parts[3], &QUALITY_FLAG),
            avg_rh: RecordValue::<f64>::new(&parts[4], "%", 10f64),
            avg_rh_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
            avg_rh_quality_flag: CodeRecord::new(&parts[6], &QUALITY_FLAG),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CI1 {
    hourly_min_air_temp: Option<RecordValue<f64>>,
    hourly_min_air_temp_quality_code: CodeRecord,
    hourly_min_air_temp_quality_flag: CodeRecord,
    hourly_max_air_temp: Option<RecordValue<f64>>,
    hourly_max_air_temp_quality_code: CodeRecord,
    hourly_max_air_temp_quality_flag: CodeRecord,
    std_dev_air_temp: Option<RecordValue<f64>>,
    std_dev_air_temp_quality_code: CodeRecord,
    std_dev_air_temp_quality_flag: CodeRecord,
    std_dev_rh: Option<RecordValue<f64>>,
    std_dev_rh_quality_code: CodeRecord,
    std_dev_rh_quality_flag: CodeRecord,
}
impl FromStr for CI1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CI1 {
            hourly_min_air_temp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            hourly_min_air_temp_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            hourly_min_air_temp_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
            hourly_max_air_temp: RecordValue::<f64>::new(&parts[3], "°C", 10f64),
            hourly_max_air_temp_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            hourly_max_air_temp_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG),
            std_dev_air_temp: RecordValue::<f64>::new(&parts[6], "°C", 10f64),
            std_dev_air_temp_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
            std_dev_air_temp_quality_flag: CodeRecord::new(&parts[8], &QUALITY_FLAG),
            std_dev_rh: RecordValue::<f64>::new(&parts[9], "%", 10f64),
            std_dev_rh_quality_code: CodeRecord::new(&parts[10], &QUALITY_CODES),
            std_dev_rh_quality_flag: CodeRecord::new(&parts[11], &QUALITY_FLAG),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CN1 {
    battery_voltage: Option<RecordValue<f64>>,
    battery_voltage_quality_code: CodeRecord,
    battery_voltage_quality_flag: CodeRecord,
    batter_voltage_full_load: Option<RecordValue<f64>>,
    battery_voltage_full_load_quality_code: CodeRecord,
    battery_voltage_full_load_quality_flag: CodeRecord,
    battery_voltage_data_logger: Option<RecordValue<f64>>,
    battery_voltage_data_logger_quality_code: CodeRecord,
    battery_voltage_data_logger_quality_flag: CodeRecord,
}
impl FromStr for CN1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CN1 {
            battery_voltage: RecordValue::<f64>::new(&parts[0], "V", 10f64),
            battery_voltage_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            battery_voltage_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
            batter_voltage_full_load: RecordValue::<f64>::new(&parts[3], "V", 10f64),
            battery_voltage_full_load_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            battery_voltage_full_load_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG),
            battery_voltage_data_logger: RecordValue::<f64>::new(&parts[6], "V", 10f64),
            battery_voltage_data_logger_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
            battery_voltage_data_logger_quality_flag: CodeRecord::new(&parts[8], &QUALITY_FLAG),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CN2 {
    tinlet_temp: Option<RecordValue<f64>>,
    tinlet_temp_quality_code: CodeRecord,
    tinlet_temp_quality_flag: CodeRecord,
    tinlet_max_temp: Option<RecordValue<f64>>,
    tinlet_max_temp_quality_code: CodeRecord,
    tinlet_max_temp_quality_flag: CodeRecord,
    door_open_time: Option<RecordValue<i32>>,
    door_open_time_quality_code: CodeRecord,
    door_open_time_quality_flag: CodeRecord,
}
impl FromStr for CN2 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(CN2 {
            tinlet_temp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            tinlet_temp_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            tinlet_temp_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
            tinlet_max_temp: RecordValue::<f64>::new(&parts[3], "°C", 10f64),
            tinlet_max_temp_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            tinlet_max_temp_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG),
            door_open_time: RecordValue::<i32>::new(&parts[6], "minutes", 1),
            door_open_time_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
            door_open_time_quality_flag: CodeRecord::new(&parts[8], &QUALITY_FLAG),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CN3 {
    ref_res_avg: Option<RecordValue<f64>>,
    ref_res_avg_quality_code: CodeRecord,
    ref_res_avg_quality_flag: CodeRecord,
    d_signature: Option<RecordValue<f64>>,
    d_signature_quality_code: CodeRecord,
    d_signature_quality_flag: CodeRecord,
}
impl FromStr for CN3 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(CN3 {
            ref_res_avg: RecordValue::<f64>::new(&parts[0], "ohms", 10f64),
            ref_res_avg_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            ref_res_avg_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
            d_signature: RecordValue::<f64>::new(&parts[3], "", 10f64),
            d_signature_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            d_signature_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct CN4 {
    gague_heater_flag: CodeRecord,
    gague_heater_quality_code: CodeRecord,
    gague_heater_quality_flag: CodeRecord,
    door_flag: CodeRecord,
    door_flag_quality_code: CodeRecord,
    door_flag_quality_flag: CodeRecord,
    fort_trans: Option<RecordValue<f64>>,
    fort_trans_quality_code: CodeRecord,
    refl_trans: Option<RecordValue<f64>>,
    refl_trans_quality_code: CodeRecord,
    refl_trans_quality_flag: CodeRecord,
}
impl FromStr for CN4 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        let door_flag = isize::from_str_radix(&parts[3], 2).unwrap();
        let message = match door_flag {
            0 => "closed",
            1..=8192 => "open",
            9999 => "missing",
            _ => "missing",
        };
        Ok(CN4 {
            gague_heater_flag: CodeRecord::new(&parts[0], &GAGUE_HEATER_FLAG),
            gague_heater_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            gague_heater_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG),
            door_flag: CodeRecord {
                value: door_flag.to_string(),
                description: message.to_string(),
            },
            door_flag_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            door_flag_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG),
            fort_trans: RecordValue::<f64>::new(&parts[6], "watts", 10f64),
            fort_trans_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
            refl_trans: RecordValue::<f64>::new(&parts[8], "watts", 10f64),
            refl_trans_quality_code: CodeRecord::new(&parts[9], &QUALITY_CODES),
            refl_trans_quality_flag: CodeRecord::new(&parts[10], &QUALITY_FLAG),
        })
    }
}
