use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

pub static TENDENCY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Increasing, then decreasing; atmospheric pressure the same or higher than 3 hours ago",
    "1" => "Increasing then steady; or increasing, then increasing more slowly; atmospheric pressure now higher than 3 hours ago",
    "2" => "Increasing (steadily or unsteadily); atmospheric pressure now higher than 3 hours ago",
    "3" => "Decreasing or steady, then increasing; or increasing, then increasing more rapidly; atmospheric pressure now higher than 3 hours ago",
    "4" => "Steady; atmospheric pressure the same as 3 hours ago",
    "5" => "Decreasing, then increasing; atmospheric pressure the same or lower than 3 hours ago",
    "6" => "Decreasing, then steady; or decreasing, then decreasing more slowly; atmospheric pressure now lower than 3 hours ago",
    "7" => "Decreasing (steadily or unsteadily); atmospheric pressure now lower than 3 hours ago",
};

pub static ISOBARIC_LEVEL_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "1000 hectopascals",
    "2" => "925 hectopascals",
    "3" => "850 hectopascals",
    "4" => "700 hectopascals",
    "5" => "500 hectopascals",
    "9" => "Missing",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MA1 {
    altimeter_setting_rate: Option<RecordValue<f64>>,
    altimeter_quality_code: CodeRecord,
    station_pressure_rate: Option<RecordValue<f64>>,
    station_pressure_quality_code: CodeRecord,
}
impl FromStr for MA1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MA1 {
            altimeter_setting_rate: RecordValue::<f64>::new(&parts[0], "hPa", 10f64),
            altimeter_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            station_pressure_rate: RecordValue::<f64>::new(&parts[2], "hPa", 10f64),
            station_pressure_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MD1 {
    tendency_code: CodeRecord,
    tendency_quality_code: CodeRecord,
    three_hour_quantity: Option<RecordValue<f64>>,
    three_hour_quantity_quality_code: CodeRecord,
    twenty_four_hour_quantity: Option<RecordValue<f64>>,
    twenty_four_hour_quantity_quality_code: CodeRecord,
}
impl FromStr for MD1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MD1 {
            tendency_code: CodeRecord::new(&parts[0], &TENDENCY_CODES),
            tendency_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            three_hour_quantity: RecordValue::<f64>::new(&parts[2], "hPa", 10f64),
            three_hour_quantity_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            twenty_four_hour_quantity: RecordValue::<f64>::new(&parts[4], "hPa", 10f64),
            twenty_four_hour_quantity_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct ME1 {
    code: CodeRecord,
    height_dimension: Option<RecordValue<f64>>,
    height_dimension_quality_code: CodeRecord,
}
impl FromStr for ME1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(ME1 {
            code: CodeRecord::new(&parts[0], &ISOBARIC_LEVEL_CODE),
            height_dimension: RecordValue::<f64>::new(&parts[1], "m", 1f64),
            height_dimension_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
        })
    }
}

// MF1 and MG1 are the same
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MF1 {
    avg_station_pressure_day: Option<RecordValue<f64>>,
    avg_station_pressure_day_quality_code: CodeRecord,
    avg_sea_level_pressure_day: Option<RecordValue<f64>>,
    avg_sea_level_pressure_day_quality_code: CodeRecord,
}
impl FromStr for MF1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MF1 {
            avg_station_pressure_day: RecordValue::<f64>::new(&parts[0], "hPa", 10f64),
            avg_station_pressure_day_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            avg_sea_level_pressure_day: RecordValue::<f64>::new(&parts[2], "hPa", 10f64),
            avg_sea_level_pressure_day_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MG1 {
    avg_station_pressure_day: Option<RecordValue<f64>>,
    avg_station_pressure_day_quality_code: CodeRecord,
    avg_sea_level_pressure_day: Option<RecordValue<f64>>,
    avg_sea_level_pressure_day_quality_code: CodeRecord,
}
impl FromStr for MG1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MG1 {
            avg_station_pressure_day: RecordValue::<f64>::new(&parts[0], "hPa", 10f64),
            avg_station_pressure_day_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            avg_sea_level_pressure_day: RecordValue::<f64>::new(&parts[2], "hPa", 10f64),
            avg_sea_level_pressure_day_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MH1 {
    avg_station_pressure_month: Option<RecordValue<f64>>,
    avg_station_pressure_month_quality_code: CodeRecord,
    avg_sea_level_pressure_month: Option<RecordValue<f64>>,
    avg_sea_level_pressure_month_quality_code: CodeRecord,
}
impl FromStr for MH1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MH1 {
            avg_station_pressure_month: RecordValue::<f64>::new(&parts[0], "hPa", 10f64),
            avg_station_pressure_month_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            avg_sea_level_pressure_month: RecordValue::<f64>::new(&parts[2], "hPa", 10f64),
            avg_sea_level_pressure_month_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MK1 {
    max_sea_level_pressure_month: Option<RecordValue<f64>>,
    max_sea_level_pressure_month_date_time: Option<RecordValue<isize>>,
    max_sea_level_pressure_month_quality_code: CodeRecord,
    min_sea_level_pressure_month: Option<RecordValue<f64>>,
    min_sea_level_pressure_month_date_time: Option<RecordValue<isize>>,
    min_sea_level_pressure_month_quality_code: CodeRecord,
}
impl FromStr for MK1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(MK1 {
            max_sea_level_pressure_month: RecordValue::<f64>::new(&parts[0], "hPa", 10f64),
            max_sea_level_pressure_month_date_time: RecordValue::<isize>::new(&parts[1], "", 1),
            max_sea_level_pressure_month_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            min_sea_level_pressure_month: RecordValue::<f64>::new(&parts[3], "hPa", 10f64),
            min_sea_level_pressure_month_date_time: RecordValue::<isize>::new(&parts[4], "", 1),
            min_sea_level_pressure_month_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}
