use crate::fields::codes::{CodeRecord, DL_QUALITY_CODES, DL_QUALITY_FLAG, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

pub static CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "N" => "Minimum temperature",
    "M" => "Maximum temperature",
    "O" => "Estimated minimum temperature",
    "P" => "Estimated maximum temperature",
    "9" => "Missing",
};
pub static CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "The value occurred on other dates in addition to those listed",
    "9" => "Missing or not applicable",
};
pub static HEATING_COOLING_DAY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "H" => "Heating Degree Days",
    "C" => "Cooling Degree Days",
};

pub static AVERAGE_DEW_POINT_AND_WET_BULB_TEMPERATURE_CODE : phf::Map<&'static str, &'static str> = phf_map! {
    "D" => "Average dew point temperature",
    "W" => "Average wet bulb temperature",
    "9" => "missing",
};
pub static AVERAGE_DEW_POINT_AND_WET_BULB_TEMPERATURE_DERIVED_CODE : phf::Map<&'static str, &'static str> = phf_map! {
 "D" => "Derived from hourly values",
 "9" => "missing",
};
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//ka1-ka4
pub struct KAX {
    period_quantity: Option<RecordValue<f64>>,
    code: CodeRecord,
    air_temperature: Option<RecordValue<f64>>,
    air_temperature_quality_code: CodeRecord,
}
impl FromStr for KAX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KAX {
            period_quantity: RecordValue::<f64>::new(&parts[0], "hours", 10f64),
            code: CodeRecord::new(&parts[1], &CODES),
            air_temperature: RecordValue::<f64>::new(&parts[2], "°C", 10f64),
            air_temperature_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

//KC1-KC2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct KCX {
    code: CodeRecord,
    condition_code: CodeRecord,
    temperature: Option<RecordValue<f64>>,
    date: Option<RecordValue<i32>>,
    temperature_quality_code: CodeRecord,
}
impl FromStr for KCX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KCX {
            code: CodeRecord::new(&parts[0], &CODES),
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            temperature: RecordValue::<f64>::new(&parts[2], "°C", 10f64),
            date: RecordValue::<i32>::new(&parts[3], "", 1),
            temperature_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}

//kd1-kd2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct KDX {
    period_quantity: Option<RecordValue<isize>>,
    code: CodeRecord,
    /// The total heating or cooling degree days for a given period, typically for the day or month, as reported by the station (ie,
    /// not derived from other data fields). These data use the 65-degree Fahrenheit base as raditionally used for degree days.
    value: Option<RecordValue<isize>>,
    quality_code: CodeRecord,
}
impl FromStr for KDX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KDX {
            period_quantity: RecordValue::<isize>::new(&parts[0], "hours", 1isize),
            code: CodeRecord::new(&parts[1], &HEATING_COOLING_DAY_CODES),
            value: RecordValue::<isize>::new(&parts[2], "Heating or Cooling Degree Days", 1isize),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct KE1 {
    max_temp_32_f_days: Option<RecordValue<isize>>,
    max_temp_32_f_days_quality_code: CodeRecord,
    max_temp_90_f_days: Option<RecordValue<isize>>,
    max_temp_90_f_days_quality_code: CodeRecord,
    min_temp_32_f_days: Option<RecordValue<isize>>,
    min_temp_32_f_days_quality_code: CodeRecord,
    min_temp_0_f_days: Option<RecordValue<isize>>,
}
impl FromStr for KE1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KE1 {
            max_temp_32_f_days: RecordValue::<isize>::new(&parts[0], "Days", 1isize),
            max_temp_32_f_days_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            max_temp_90_f_days: RecordValue::<isize>::new(&parts[2], "Days", 1isize),
            max_temp_90_f_days_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            min_temp_32_f_days: RecordValue::<isize>::new(&parts[4], "Days", 1isize),
            min_temp_32_f_days_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
            min_temp_0_f_days: RecordValue::<isize>::new(&parts[6], "Days", 1isize),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct KF1 {
    air_temp: Option<RecordValue<f64>>,
    air_temp_quality_code: CodeRecord,
}
impl FromStr for KF1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KF1 {
            air_temp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            air_temp_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}

//KG1-KG2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct KGX {
    period_quantity: Option<RecordValue<isize>>,
    code: CodeRecord,
    temp: Option<RecordValue<f64>>,
    derived_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for KGX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(KGX {
            period_quantity: RecordValue::<isize>::new(&parts[0], "hours", 1isize),
            code: CodeRecord::new(&parts[1], &AVERAGE_DEW_POINT_AND_WET_BULB_TEMPERATURE_CODE),
            temp: RecordValue::<f64>::new(&parts[2], "°C", 10f64),
            derived_code: CodeRecord::new(&parts[3], &AVERAGE_DEW_POINT_AND_WET_BULB_TEMPERATURE_DERIVED_CODE),
            quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}