use std::num::ParseFloatError;
use std::str::FromStr;

use crate::util::get_parts;
use crate::{model::RecordValue, util::is_null};
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;

use super::codes::{CodeRecord, BOOL_CODES, QUALITY_CODES};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GeoPhysicalPoint(Option<f64>);

impl FromStr for GeoPhysicalPoint {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //parse string as float
        if is_null(s) {
            Ok(GeoPhysicalPoint(None))
        } else {
            let f = s.parse::<f64>()?;
            Ok(GeoPhysicalPoint(Some(f)))
        }
    }
}

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
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct Wind {
    /// The angle, measured in a clockwise direction, between true north and the direction from which the wind is blowing.
    direction_angle: Option<RecordValue<i32>>,
    /// The code that denotes a quality status of a reported WIND-OBSERVATION direction angle.
    direction_quality_code: CodeRecord,
    /// The code that denotes the character of the WIND-OBSERVATION.
    type_code: CodeRecord,
    /// The rate of horizontal travel of air past a fixed point.
    speed_rate: Option<RecordValue<f64>>,
}

impl FromStr for Wind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(Wind {
            direction_angle: RecordValue::<i32>::new(&parts[0], "°", 1),
            direction_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            type_code: CodeRecord::new(&parts[2], &WIND_OBSERVATION_TYPE_CODES),
            speed_rate: RecordValue::<f64>::new(&parts[3], "m/s", 1000f64),
        })
    }
}
/// The code that denotes the method used to determine the ceiling.
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
///The height above ground level (AGL) of the lowest cloud or obscuring phenomena layer aloft with 5/8 or more summation total sky cover,
///which may be predominantly opaque, or the vertical visibility into a surface-based obstruction.
///Unlimited = 22000.
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct Ceiling {
    height: Option<RecordValue<i32>>,
    quality_code: CodeRecord,
    /// The code that denotes the method used to determine the ceiling.
    determination_code: CodeRecord,
    /// The code that represents whether the 'Ceiling and Visibility Okay' (CAVOK) condition has been reported.
    cavok: CodeRecord,
}

impl FromStr for Ceiling {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(Ceiling {
            height: RecordValue::<i32>::new(&parts[0], "m", 1),
            quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            determination_code: CodeRecord::new(&parts[2], &CEILING_DETERMINATION_CODE),
            cavok: CodeRecord::new(&parts[3], &BOOL_CODES),
        })
    }
}

// The code that denotes whether or not the reported visibility is variable.
pub static VISIBILITY_VARIABILITY_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "N" => "Not variable",
    "V" => "Variable",
    "9" => "Missing",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct Visibility {
    /// The horizontal distance at which an object can be seen and identified.
    distance: Option<RecordValue<i32>>,
    distance_quality_code: CodeRecord,
    variability: CodeRecord,
    variability_quality_code: CodeRecord,
}

impl FromStr for Visibility {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(Visibility {
            distance: RecordValue::<i32>::new(&parts[0], "m", 1),
            distance_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            variability: CodeRecord::new(&parts[2], &VISIBILITY_VARIABILITY_CODE),
            variability_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct Temperature {
    /// The temperature of the air.
    air_temperature: Option<RecordValue<i32>>,
    air_temperature_quality_code: CodeRecord,
}
impl FromStr for Temperature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(Temperature {
            air_temperature: RecordValue::<i32>::new(&parts[0], "°C", 10),
            air_temperature_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct Dew {
    ///The temperature to which a given parcel of air must be cooled at constant pressure and water vapor content in order for saturation to occur.
    dew_point_temperature: Option<RecordValue<i32>>,
    dew_point_temperature_quality_code: CodeRecord,
}
impl FromStr for Dew {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(Dew {
            dew_point_temperature: RecordValue::<i32>::new(&parts[0], "°C", 10),
            dew_point_temperature_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}

/// The air pressure relative to Mean Sea Level (MSL).
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct SeaLevelPressure {
    pressure: Option<RecordValue<i32>>,
    pressure_quality_code: CodeRecord,
}

impl FromStr for SeaLevelPressure {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(SeaLevelPressure {
            pressure: RecordValue::<i32>::new(&parts[0], "hPa", 10),
            pressure_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
