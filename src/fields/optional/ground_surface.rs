use crate::fields::codes::{CodeRecord, DL_QUALITY_CODES, DL_QUALITY_FLAG, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

/// The code that denotes the physical condition of the ground's surface.
///  NOTE: Code values 10-19 indicate the state of the ground without snow or measurable ice cover.
pub static OBSERVATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
   "00" => "Surface of ground dry (no appreciable amount of dust or loose sand)",
   "01" => "Surface of ground dry (without cracks and no appreciable amount of dust or loose sand and without snow or measurable ice cover)",
   "02" => "Extremely dry with cracks (without snow or measurable ice cover)",
   "03" => "Loose dry dust or sand not covering ground completely (without snow or measurable ice cover)",
   "04" => "Loose dry dust or sand covering more than one-half of ground (but not completely)",
   "05" => "Loose dry dust or sand covering ground completely",
   "06" => "Thin cover of loose dry dust or sand covering ground completely (without snow or measurable ice cover)",
   "07" => "Moderate or thick cover of loose dry dust or sand covering ground completely (without snow or measurable ice cover)",
   "08" => "Surface of ground moist",
   "09" => "Surface of ground moist (without snow or measurable ice cover)",
   "10" => "Surface of ground wet (standing water in small or large pools on surface)",
   "11" => "Surface of ground wet (standing water in small or large pools on surface without snow or measurable ice cover)",
   "12" => "Flooded (without snow or measurable ice cover)",
   "13" => "Surface of ground frozen",
   "14" => "Surface of ground frozen (without snow or measurable ice cover)",
   "15" => "Glaze or ice on ground, but no snow or melting snow",
   "16" => "Glaze on ground (without snow or measurable ice cover)",
   "17" => "Ground predominantly covered by ice",
   "18" => "Snow or melting snow (with or without ice) covering less than one-half of the ground",
   "19" => "Snow or melting snow (with or without ice) covering more than one-half of the ground but ground not completely covered",
   "20" => "Snow or melting snow (with or without ice) covering ground completely",
   "21" => "Loose dry snow covering less than one-half of the ground",
   "22" => "Loose dry snow covering at least one half of the ground (but not completely)",
   "23" => "Even layer of loose dry snow covering ground completely",
   "24" => "Uneven layer of loose dry snow covering ground completely",
   "25" => "Compact or wet snow (with or without ice) covering less than one-half of the ground",
   "26" => "Compact or wet snow (with or without ice) covering at least one-half of the ground but ground not completely covered",
   "27" => "Even layer of compact or wet snow covering ground completely",
   "28" => "Uneven layer of compact or wet snow covering ground completely",
   "29" => "Snow covering ground completely; deep drifts",
   "30" => "Lose dry dust or sand covering one-half of the ground (but not completely)",
   "31" => "Loose dry snow, dust or sand covering ground completely",
   "99" => "Missing",
};

pub static CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
 "1" => "No special conditions",
 "2" => "Data will be included in subsequent observation",
 "3" => "Data are accumulated from previous observation(s), so cover a longer than typical time period",
 "9" => "Missing",
};
///GROUND-SURFACE-OBSERVATION identifier
///The identifier that denotes the availability of a GROUND-SURFACE-OBSERVATION.

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct IA1 {
    observation_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for IA1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(IA1 {
            observation_code: CodeRecord::new(&parts[0], &OBSERVATION_CODES),
            quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
///GROUND-SURFACE-OBSERVATION minimum-temperature identifier
///The identifier that denotes the availability of GROUND-SURFACE-OBSERVATION minimum temperature data.
pub struct IA2 {
    min_temp_period: Option<RecordValue<f64>>,
    min_temp: Option<RecordValue<f64>>,
    min_temp_quality_code: CodeRecord,
}

impl FromStr for IA2 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(IA2 {
            min_temp_period: RecordValue::<f64>::new(&parts[0], "h", 1f64),
            min_temp: RecordValue::<f64>::new(&parts[1], "°C", 10f64),
            min_temp_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
///Hourly Surface Temperature Section identifier
///The identifier that indicates an hourly observation of surface temperature as measured by a radiation sensor for the
///ground surface. This section appears in the last ISD record of the hour.
pub struct IB1 {
    /// The hourly average surface temperature.
    surftemp: Option<RecordValue<f64>>,
    /// The code that indicates ISD’s evaluation of the quality status of the hourly average surface temperature.
    surftemp_qc: CodeRecord,
    /// The code that indicates the network’s internal evaluation of the quality status of the hourly average surface temperature. Most users will find the preceding quality code SURFTEMP_QC to be the simplest and most useful quality indicator.
    surftemp_flag: CodeRecord,
    /// The minimum 10 second surface temperature for the hour.
    surftemp_min: Option<RecordValue<f64>>,
    /// The code that indicates ISD’s evaluation of the quality status of the hourly minimum surface temperature.
    surftemp_min_qc: CodeRecord,
    /// The code that indicates the network’s internal evaluation of the quality status of the hourly minimum surface temperature. Most users will find the preceding quality code SURFTEMP_MIN_QC to be the simplest and most useful quality indicator.
    surftemp_min_flag: CodeRecord,
    /// The maximum 10 second surface temperature for the hour.
    surftemp_max: Option<RecordValue<f64>>,
    // The code that indicates ISD’s evaluation of the quality status of the hourly maximum surface temperature.
    surftemp_max_qc: CodeRecord,
    ///The code that indicates the network’s internal evaluation of the quality status of the hourly maximum surface temperature. Most users will find the preceding quality code SURFTEMP_MAX_QC to be the simplest and most useful quality indicator.
    surftemp_max_flag: CodeRecord,
}
impl FromStr for IB1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(IB1 {
            surftemp: RecordValue::<f64>::new(&parts[0], "°C", 10f64),
            surftemp_qc: CodeRecord::new(&parts[1], &DL_QUALITY_CODES),
            surftemp_flag: CodeRecord::new(&parts[2], &DL_QUALITY_FLAG),
            surftemp_min: RecordValue::<f64>::new(&parts[3], "°C", 10f64),
            surftemp_min_qc: CodeRecord::new(&parts[4], &DL_QUALITY_CODES),
            surftemp_min_flag: CodeRecord::new(&parts[5], &DL_QUALITY_FLAG),
            surftemp_max: RecordValue::<f64>::new(&parts[6], "°C", 10f64),
            surftemp_max_qc: CodeRecord::new(&parts[7], &DL_QUALITY_CODES),
            surftemp_max_flag: CodeRecord::new(&parts[8], &DL_QUALITY_FLAG),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct IC1 {
    time_period: Option<RecordValue<isize>>,
    wind_movement: Option<RecordValue<isize>>,
    wind_movement_condition_code: CodeRecord,
    wind_movement_quality_code: CodeRecord,
    evaporation_data: Option<RecordValue<f64>>,
    evaporation_condition_code: CodeRecord,
    evaporation_quality_code: CodeRecord,
    max_pan_water_temp: Option<RecordValue<f64>>,
    max_pan_water_temp_condition_code: CodeRecord,
    max_pan_water_temp_quality_code: CodeRecord,
    min_pan_water_temp: Option<RecordValue<f64>>,
    min_pan_water_temp_condition_code: CodeRecord,
    min_pan_water_temp_quality_code: CodeRecord,
}

impl FromStr for IC1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(IC1 {
            time_period: RecordValue::<isize>::new(&parts[0], "h", 1isize),
            wind_movement: RecordValue::<isize>::new(&parts[1], "mi", 1isize),
            wind_movement_condition_code: CodeRecord::new(&parts[2], &CONDITION_CODES),
            wind_movement_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            evaporation_data: RecordValue::<f64>::new(&parts[4], "in", 1f64),
            evaporation_condition_code: CodeRecord::new(&parts[5], &CONDITION_CODES),
            evaporation_quality_code: CodeRecord::new(&parts[6], &QUALITY_CODES),
            max_pan_water_temp: RecordValue::<f64>::new(&parts[7], "°C", 10f64),
            max_pan_water_temp_condition_code: CodeRecord::new(&parts[8], &CONDITION_CODES),
            max_pan_water_temp_quality_code: CodeRecord::new(&parts[9], &QUALITY_CODES),
            min_pan_water_temp: RecordValue::<f64>::new(&parts[10], "°C", 10f64),
            min_pan_water_temp_condition_code: CodeRecord::new(&parts[11], &CONDITION_CODES),
            min_pan_water_temp_quality_code: CodeRecord::new(&parts[12], &QUALITY_CODES),
        })
    }
}
