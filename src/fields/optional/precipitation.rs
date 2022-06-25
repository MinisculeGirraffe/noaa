use std::str::FromStr;

use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::util::{get_parts, parse_null};
use crate::{model::RecordValue, util::is_null};
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;

pub static CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {

 "1" => "Measurement impossible or inaccurate",
 "2" => "Trace",
 "3" => "Begin accumulated period (precipitation amount missing until end of accumulated period)",
 "4" => "End accumulated period",
 "5" => "Begin deleted period (precipitation amount missing due to data problem)",
 "6" => "End deleted period",
 "7" => "Begin missing period",
 "8" => "End missing period",
 "E" => "Estimated data value (eg, from nearby station)",
 "I" => "Incomplete precipitation amount, excludes one or more missing reports, such as one or more 15-minute reports not included in the 1-hour precipitation total",
 "J" => "Incomplete precipitation amount, excludes one or more erroneous reports, such as one or more 1-hour precipitation amounts excluded from the 24-hour total",
 "9" => "Missing",
};

pub static DURATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
 "0" => "Lasted less than 1 hour",
 "1" => "Lasted 1 - 3 hours",
 "2" => "Lasted 3 - 6 hours",
 "3" => "Lasted more than 6 hours",
 "9" => "Missing",
};

pub static CHARACTERISTIC_CODES: phf::Map<&'static str, &'static str> = phf_map! {
 "C" => "Continuous",
 "I" => "Intermittent",
 "9" => "Missing",
};

pub static DISCREPANCY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Reported amount of precipitation and reported weather agree",
    "1" => "Precipitation missing or not reported and none inferred by weather",
    "2" => "Precipitation missing, but precipitation inferred by weather",
    "3" => "Precipitation reported, but none inferred by weather",
    "4" => "Zero precipitation reported, but precipitation inferred by weather",
    "5" => "Zero precipitation reported, no precipitation inferred and precipitation not occurring at the reporting station",
    "9" => "Missing",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AAX {
    period_quantity: Option<RecordValue<i8>>,
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}

impl FromStr for AAX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AAX {
            period_quantity: RecordValue::new(&parts[0], "Hours", 1),
            depth_dimension: RecordValue::new(&parts[1], "mm", 10f64),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AB1 {
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}

impl FromStr for AB1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AB1 {
            depth_dimension: RecordValue::new(&parts[0], "mm", 10f64),
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AC1 {
    duration_code: CodeRecord,
    characteristic_code: CodeRecord,
    quality_code: CodeRecord,
}

impl FromStr for AC1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(AC1 {
            duration_code: CodeRecord::new(&parts[0], &DURATION_CODES),
            characteristic_code: CodeRecord::new(&parts[1], &CHARACTERISTIC_CODES),
            quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AD1 {
    depth_dimension: Option<RecordValue<f64>>,
    dates_of_occurrence: Vec<String>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}

impl FromStr for AD1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        let mut days: Vec<String> = Vec::new();
        // grow string at current index and insert a - in the middle
        for i in 2..4 {
            if is_null(&parts[i]) {
                continue;
            };
            let len = parts[i].len();
            let mut c = parts[i].chars().collect::<Vec<char>>();
            c.insert(len / 2, '-');
            days.push(c.iter().collect::<String>());
        }

        Ok(AD1 {
            depth_dimension: RecordValue::new(&parts[0], "mm", 10f64),
            dates_of_occurrence: days,
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AE1 {
    days_01: Option<RecordValue<i8>>,
    days_01_quality: CodeRecord,
    days_10: Option<RecordValue<i8>>,
    days_10_quality: CodeRecord,
    days_50: Option<RecordValue<i8>>,
    days_50_quality: CodeRecord,
    days_100: Option<RecordValue<i8>>,
    days_100_quality: CodeRecord,
}

impl FromStr for AE1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AE1 {
            days_01: RecordValue::new(&parts[0], "Days", 1),
            days_01_quality: CodeRecord::new(&parts[1], &QUALITY_CODES),
            days_10: RecordValue::new(&parts[2], "Days", 1),
            days_10_quality: CodeRecord::new(&parts[3], &QUALITY_CODES),
            days_50: RecordValue::new(&parts[4], "Days", 1),
            days_50_quality: CodeRecord::new(&parts[5], &QUALITY_CODES),
            days_100: RecordValue::new(&parts[6], "Days", 1),
            days_100_quality: CodeRecord::new(&parts[7], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AG1 {
    discrepancy_code: CodeRecord,
    estimated_water_depth_dimension: Option<RecordValue<f64>>,
}
impl FromStr for AG1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AG1 {
            discrepancy_code: CodeRecord::new(&parts[0], &DISCREPANCY_CODES),
            estimated_water_depth_dimension: RecordValue::new(&parts[1], "mm", 1f64),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AHX {
    period_quantity: Option<RecordValue<i8>>,
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    end_date_time: Option<String>,
    quality_code: CodeRecord,
}

impl FromStr for AHX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AHX {
            period_quantity: RecordValue::new(&parts[0], "minutes", 1),
            depth_dimension: RecordValue::new(&parts[1], "mm", 10f64),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            end_date_time: parse_null(&parts[3]),
            quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AIX {
    period_quantity: Option<RecordValue<i16>>,
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    end_date_time: Option<String>,
    quality_code: CodeRecord,
}
impl FromStr for AIX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AIX {
            period_quantity: RecordValue::new(&parts[0], "minutes", 1),
            depth_dimension: RecordValue::new(&parts[1], "mm", 10f64),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            end_date_time: parse_null(&parts[3]),
            quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AJ1 {
    depth_dimension: Option<RecordValue<i16>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
    equivalent_water_depth_dimension: Option<RecordValue<f64>>,
    equivalent_water_condition_code: CodeRecord,
    equivalent_water_condition_quality_code: CodeRecord,
}

impl FromStr for AJ1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AJ1 {
            depth_dimension: RecordValue::new(&parts[0], "cm", 1),
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            equivalent_water_depth_dimension: RecordValue::new(&parts[3], "mm", 10f64),
            equivalent_water_condition_code: CodeRecord::new(&parts[4], &CONDITION_CODES),
            equivalent_water_condition_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AK1 {
    depth_dimension: Option<RecordValue<i16>>,
    condition_code: CodeRecord,
    dates_of_occourence: [String; 3],
    quality_code: CodeRecord,
}

impl FromStr for AK1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AK1 {
            depth_dimension: RecordValue::new(&parts[0], "cm", 1),
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            dates_of_occourence: [parts[2].clone(), parts[3].clone(), parts[4].clone()],
            quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct ALX {
    period_quantity: Option<RecordValue<i8>>,
    depth_dimension: Option<RecordValue<i16>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for ALX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(ALX {
            period_quantity: RecordValue::new(&parts[0], "hours", 1),
            depth_dimension: RecordValue::new(&parts[1], "cm", 1),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AM1 {
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    dates_of_occourence: [String; 3],
    quality_code: CodeRecord,
}
impl FromStr for AM1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AM1 {
            depth_dimension: RecordValue::new(&parts[0], "cm", 10f64),
            condition_code: CodeRecord::new(&parts[1], &CONDITION_CODES),
            //TODO: parse dates
            dates_of_occourence: [
                parts[2].to_string(),
                parts[3].to_string(),
                parts[4].to_string(),
            ],
            quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AN1 {
    period_quantity: Option<RecordValue<i8>>,
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for AN1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AN1 {
            period_quantity: RecordValue::new(&parts[0], "hours", 1),
            depth_dimension: RecordValue::new(&parts[1], "cm", 10f64),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AOX {
    period_quantity: Option<RecordValue<i16>>,
    depth_dimension: Option<RecordValue<f64>>,
    condition_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for AOX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);

        Ok(AOX {
            period_quantity: RecordValue::new(&parts[0], "minutes", 1),
            depth_dimension: RecordValue::new(&parts[1], "mm", 10f64),
            condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
// apx intentionally not implemented
