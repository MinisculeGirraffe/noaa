use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{ Deserialize, Deserializer, Serialize};

use super::codes::{CodeRecord, BOOL_CODES, QUALITY_CODES};

pub static WIND_OBSERVATION_TYPE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "Abridged Beaufort",
    "B" => "Beaufort",
    "C" => "Calm",
    "H" => "5-Minute Average Speed",
    "N" => "Normal",
    "R" => "60-Minute Average Speed",
    "Q" => "Squall",
    "T" => "180 Minute Average Speed",
    "V" => "Variable",
    "9" => "Missing",
};

#[derive(Serialize, Debug, PartialEq)]
pub struct Wind {
    direction_angle: RecordValue<i32>,
    direction_quality_code: CodeRecord,
    type_code: CodeRecord,
    speed_rate: RecordValue<f64>,
}

impl<'de> Deserialize<'de> for Wind {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut parts = get_parts(d, 14)?;

        Ok(Wind {
            direction_angle: RecordValue::<i32>::new(&parts[0], "°", 1),
            direction_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            type_code: CodeRecord::new(&parts[2], &WIND_OBSERVATION_TYPE_CODES),
            speed_rate: RecordValue::<f64>::new(&parts[3], "m/s", 1000f64),
        })
    }
}

pub static CEILING_DETERMINATION_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "Aircraft",
    "B" => "Balloon",
    "C" => "Statistically derived",
    "D" => "Persistent cirriform ceiling (pre-1950 data)",
    "E" => "Estimated",
    "M" => "Measured",
    "P" => "Precipitation ceiling (pre-1950 data)",
    "R" => "Radar",
    "S" => "ASOS augmented",
    "U" => "Unknown ceiling (pre-1950 data)",
    "V" => "Variable ceiling (pre-1950 data)",
    "W" => "Obscured",
    "9" => "Missing",
};

#[derive(Serialize, Debug, PartialEq)]
pub struct Ceiling {
    height: RecordValue<i32>,
    quality_code: CodeRecord,
    determination_code: CodeRecord,
    cavok: CodeRecord,
}

impl<'de> Deserialize<'de> for Ceiling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parts = get_parts(deserializer, 11)?;

        Ok(Ceiling {
            height: RecordValue::<i32>::new(&parts[0], "m", 1),
            quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            determination_code: CodeRecord::new(&parts[2], &CEILING_DETERMINATION_CODE),
            cavok: CodeRecord::new(&parts[3], &BOOL_CODES),
        })
    }
}

pub static VISIBILITY_VARIABILITY_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "N" => "Not variable",
    "V" => "Variable",
    "9" => "Missing",
};

#[derive(Serialize, Debug, PartialEq)]
pub struct Visibility {
    distance: RecordValue<i32>,
    distance_quality_code: CodeRecord,
    variability: CodeRecord,
    variability_quality_code: CodeRecord,
}

impl<'de> Deserialize<'de> for Visibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parts = get_parts(deserializer, 12)?;

        Ok(Visibility {
            distance: RecordValue::<i32>::new(&parts[0], "m", 1),
            distance_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            variability: CodeRecord::new(&parts[2], &VISIBILITY_VARIABILITY_CODE),
            variability_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Temprature {
    air_temperature: RecordValue<i32>,
    air_temperature_quality_code: CodeRecord,
}

impl<'de> Deserialize<'de> for Temprature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parts = get_parts(deserializer, 7)?;

        Ok(Temprature {
            air_temperature: RecordValue::<i32>::new(&parts[0], "°C", 10),
            air_temperature_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
#[derive(Serialize, Debug, PartialEq)]
pub struct Dew {
    dew_point_temperature: RecordValue<i32>,
    dew_point_temperature_quality_code: CodeRecord,
}

impl<'de> Deserialize<'de> for Dew {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parts = get_parts(deserializer, 7)?;

        Ok(Dew {
            dew_point_temperature: RecordValue::<i32>::new(&parts[0], "°C", 10),
            dew_point_temperature_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct SeaLevelPressure {
    pressure: RecordValue<i32>,
    pressure_quality_code: CodeRecord,
}

impl<'de> Deserialize<'de> for SeaLevelPressure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parts = get_parts(deserializer, 0)?;

        Ok(SeaLevelPressure {
            pressure: RecordValue::<i32>::new(&parts[0], "hPa", 10),
            pressure_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
