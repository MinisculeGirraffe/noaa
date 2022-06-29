use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

pub static METHOD_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "M" => "Manual",
    "I" => "Instrumental",
    "9" => "Missing",
};

pub static SEA_STATE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Calm, glassy - wave height = 0 meters",
    "01" => "Calm, rippled - wave height = 0-0.1 meters",
    "02" => "Smooth, wavelets - wave height = 0.1-0.5 meters",
    "03" => "Slight, wave height = 0.5-1.25 meters",
    "04" => "Moderate - wave height 1.25-2.5 meters",
    "05" => "Rough - wave height = 2.5-4.0 meters",
    "06" => "Very rough - wave height = 4.0-6.0 meters",
    "07" => "High - wave height = 6.0-9.0 meters",
    "08" => "Very high - wave height 9.0-14.0 meters",
    "09" => "Phenomenal - wave height = over 14.0 meters",
    "99" => "Missing",
};

pub static ICE_ACCRETION_SOURCE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Icing from ocean spray",
    "2" => "Icing from fog",
    "3" => "Icing from spray and fog",
    "4" => "Icing from rain",
    "5" => "Icing from spray and rain",
    "9" => "Missing",
};

pub static ICE_ACCRETION_TENDENCY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Ice not building up",
    "1" => "Ice building up slowly",
    "2" => "Ice building up rapidly",
    "3" => "Ice melting or breaking up slowly",
    "4" => "Ice melting or breaking up rapidly",
    "9" => "Missing",
};

pub static EDGE_BEARING_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Ship in shore or flaw lead",
    "01" => "Principal ice edge towards NE",
    "02" => "Principal ice edge towards E",
    "03" => "Principal ice edge towards SE",
    "04" => "Principal ice edge towards S",
    "05" => "Principal ice edge towards SW",
    "06" => "Principal ice edge towards W",
    "07" => "Principal ice edge towards NW",
    "08" => "Principal ice edge towards N",
    "09" => "Not determined (ship in ice)",
    "10" => "Unable to report, because of darkness, lack of visibility or because only ice of land origin is visible.",
    "99" => "Missing",
};

pub static EDGE_ORIENTATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Orientation of ice edge impossible to estimate--ship outside the ice",
    "01" => "Ice edge lying in a direction NE to SW with ice situated to the NW",
    "02" => "Ice edge lying in a direction E to W with ice situated to the N",
    "03" => "Ice edge lying in a direction SE to NW with ice situated to the NE",
    "04" => "Ice edge lying in a direction S to N with ice situated to the E",
    "05" => "Ice edge lying in a direction SW to NE with ice situated to the SE",
    "06" => "Ice edge lying in a direction W to E with ice situated to the S",
    "07" => "Ice edge lying in a direction NW to SE with ice situated to the SW",
    "08" => "Ice edge lying in a direction N to S with ice situated to the W",
    "09" => "Orientation of ice edge impossible to estimate--ship inside the ice",
    "99" => "Missing",
};

pub static NON_UNIFORM_CONCENTRATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "06" => "Strips and patches of pack ice with open water between",
    "07" => "Strips and patches of close or very close pack ice with areas of lesser concentration between",
    "08" => "Fast ice with open water, very open or open pack ice to seaward of the ice boundary",
    "09" => "Fast ice with close or very close pack ice to seaward of the ice boundary",
    "99" => "Unable to report, because of darkness, lack of visibility, or because ship is more than 0.5 nautical mile away from ice edge",
};

pub static SHIP_RELATIVE_POSITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Ship in open water with floating ice in sight",
    "1" => "In open lead or fast ice",
    "2" => "In ice or within 0.5 nautical miles of ice edge",
    "9" => "Missing",
};

pub static SHIP_PENETRABILITY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Easy",
    "2" => "Difficult",
    "3" => "Beset (Surrounded so closely by sea ice that steering control is lost.)",
    "9" => "Missing",
};

pub static FORMATION_TYPE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No ice (0 may be used to report ice blink and then a direction must be reported)",
    "01" => "New ice",
    "02" => "Fast ice",
    "03" => "Pack-ice/drift-ice",
    "04" => "Packed (compact) slush or sludge",
    "05" => "Shore lead",
    "06" => "Heavy fast ice",
    "07" => "Heavy pack-ice/drift-ice",
    "08" => "Hummocked ice",
    "09" => "Icebergs-icebergs can be reported in plain language",
    "99" => "Missing",
};

pub static NAVIGATION_EFFECT_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" =>  "Navigation unobstructed",
    "01" =>  "Navigation unobstructed for steamers, difficult for sailing ships",
    "02" =>  "Navigation difficult for low-powered steamers, closed to sailing ships",
    "03" =>  "Navigation possible only for powerful steamers",
    "04" =>  "Navigation possible only for steamers constructed to withstand ice pressure",
    "05" =>  "Navigation possible with the assistance of ice-breakers",
    "06" =>  "Channel open in the solid ice",
    "07" =>  "Navigation temporarily closed",
    "08" =>  "Navigation closed",
    "09" =>  "Navigation conditions unknown, e.g., owing to bad weather",
    "99" =>  "Missing",
};

pub static ICE_TREND_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Conditions improving",
    "2" => "Conditions static",
    "3" => "Conditions worsening",
    "4" => "Conditions worsening; ice forming and floes freezing together",
    "5" => "Conditions worsening; ice under slight pressure",
    "6" => "Conditions worsening; ice under moderate or severe pressure",
    "9" => "Missing",
};

pub static DEVELOPMENT_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "New ice only (frazil ice, grease ice, slush, slugs)",
    "01" => "Nilas or ice rind, less than 10 cm thick",
    "02" => "Young ice (grey ice, grey-white ice), 10 - 30 cm thick",
    "03" => "Predominantly new and/or young ice with some first year ice",
    "04" => "Predominantly thin first year ice with some new and/or young ice",
    "05" => "All thin first year ice (30 - 70 cm thick)",
    "06" => "Predominantly medium first year ice (70 - 120 cm thick) and thick first year ice (> 120 cm thick) with some thinner (younger) first year ice",
    "07" => "All medium and thick first year ice",
    "08" => "Predominantly medium and thick first year ice with some old ice (usually more than 2 m thick)",
    "09" => "Predominantly old ice",
    "99" => "Unable to report, because of darkness, lack of visibility or because only ice of land origin is visible or because ship is more than .5 NM away from ice",
};

pub static GROWLER_BERGY_BIT_PRESENCE_CODE: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Not present",
    "1" => "Present",
    "2" => "Unknown",
    "9" => "Missing",
};

pub static ICE_PHENOMENA_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Water surface free of ice",
    "01" => "Ice along banks",
    "02" => "Ice crystals",
    "03" => "Ice slush",
    "04" => "Ice flows from tributaries entering near the river, lake or reservoir station",
    "10" => "Floating slush ice covering approximately 1/3 (up to 30%) of the water surface",
    "11" => "Floating slush ice covering about half (40% - 60%) of the water surface",
    "12" => "Floating slush ice covering more than half (70% - 100%) of the water surface",
    "20" => "Floating ice covering 10% of the water surface",
    "21" => "Floating ice covering 20% of the water surface",
    "22" => "Floating ice covering 30% of the water surface",
    "23" => "Floating ice covering 40% of the water surface",
    "24" => "Floating ice covering 50% of the water surface",
    "25" => "Floating ice covering 60% of the water surface",
    "26" => "Floating ice covering 70% of the water surface",
    "27" => "Floating ice covering 80% of the water surface",
    "28" => "Floating ice covering 90% of the water surface",
    "29" => "Floating ice covering 100% of the water surface",
    "30" => "Water surface frozen at station, free upstream",
    "31" => "Water surface frozen at station, free downstream",
    "32" => "Water surface free at station, free upstream",
    "33" => "Water surface free at station, free downstream",
    "34" => "Ice floes near the station, water surface frozen downstream",
    "35" => "Water surface frozen with breaks",
    "36" => "Water surface completely frozen over",
    "37" => "Water surface frozen over with pile-ups",
    "40" => "Ice melting along the banks",
    "41" => "Some water on the ice",
    "42" => "Ice waterlogged",
    "43" => "Water holes in the ice cover",
    "44" => "Ice moving",
    "45" => "Open water in breaks",
    "46" => "Break up (first day of movement of ice on the entire water surface)",
    "47" => "Ice broken artificially",
    "50" => "Ice jam below the station",
    "51" => "Ice jam at the station",
    "52" => "Ice jam above the station",
    "53" => "Scale and position of jam unchanged",
    "54" => "Jam has frozen solid in the same place",
    "55" => "Jam has solidified and expanded upstream",
    "56" => "Jam has solidified and moved downstream",
    "57" => "Jam is weakening",
    "58" => "Jam broken up by explosives or other methods",
    "59" => "Jam broken",
    "60" => "Fractured ice",
    "61" => "Ice piling up againgst the bank",
    "62" => "Ice carried towards the bank",
    "63" => "Band of ice less than 100 meters wide fixed to banks",
    "64" => "Band of ice less than 100 to 500 meters wide fixed to banks",
    "65" => "Band of ice wider than 500 meters fixed to banks",
    "70" => "Cracks in the ice, mainly across the line of flow",
    "71" => "Cracks along the flow line",
    "72" => "Smooth sheet of ice",
    "73" => "Ice sheet with pile-ups",
    "99" => "Missing",
};

pub static UNDER_ICE_SLUSH_CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "No slush ice",
    "1" => "Slush ice to approximately 1/3 of depth of the river, lake or reservoir",
    "2" => "Slush ice from 1/3 to 2/3 of depth of the river, lake or reservoir",
    "3" => "Slush ice to depth of the river, lake or reservoir greater than 2/3.",
    "9" => "Missing",
};
pub static WATER_LEVEL_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "B" => "much below normal",
    "H" => "high but not overflowing",
    "N" => "normal",
    "O" => "banks overflowing",
    "9" => "missing",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct UA1 {
    method_code: CodeRecord,
    wave_period_quantity: Option<RecordValue<isize>>,
    wave_height_dimension: Option<RecordValue<f64>>,
    wave_quality_code: CodeRecord,
    sea_state_code: CodeRecord,
    sea_state_quality_code: CodeRecord,
}
impl FromStr for UA1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(UA1 {
            method_code: CodeRecord::new(&parts[0], &METHOD_CODES),
            wave_period_quantity: RecordValue::<isize>::new(&parts[1], "s", 1isize),
            wave_height_dimension: RecordValue::<f64>::new(&parts[2], "m", 10f64),
            wave_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            sea_state_code: CodeRecord::new(&parts[5], &SEA_STATE_CODES),
            sea_state_quality_code: CodeRecord::new(&parts[6], &QUALITY_CODES),
        })
    }
}

// UG1-UG2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct UGX {
    period_quantity: Option<RecordValue<isize>>,
    height_dimension: Option<RecordValue<f64>>,
    direction_angle: Option<RecordValue<isize>>,
    quality_code: CodeRecord,
}
impl FromStr for UGX {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(UGX {
            period_quantity: RecordValue::<isize>::new(&parts[0], "s", 1isize),
            height_dimension: RecordValue::<f64>::new(&parts[1], "m", 10f64),
            direction_angle: RecordValue::<isize>::new(&parts[2], "°", 1isize),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct WA1 {
    source_code: CodeRecord,
    thickness_dimension: Option<RecordValue<f64>>,
    tendency_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for WA1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(WA1 {
            source_code: CodeRecord::new(&parts[0], &ICE_ACCRETION_SOURCE_CODES),
            thickness_dimension: RecordValue::<f64>::new(&parts[1], "cm", 10f64),
            tendency_code: CodeRecord::new(&parts[2], &ICE_ACCRETION_TENDENCY_CODES),
            quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct WD1 {
    edge_bearing_code: CodeRecord,
    uniform_concentration_rate: Option<RecordValue<f64>>,
    non_uniform_concentration_code: CodeRecord,
    ship_relative_position_code: CodeRecord,
    ship_penatrability_code: CodeRecord,
    ice_trend_code: CodeRecord,
    development_code: CodeRecord,
    growler_bergy_bit_presence_code: CodeRecord,
    growler_bergy_bit_quantity: Option<RecordValue<isize>>,
    iceberg_quantity: Option<RecordValue<isize>>,
    quality_code: CodeRecord,
}
impl FromStr for WD1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(WD1 {
            edge_bearing_code: CodeRecord::new(&parts[0], &EDGE_BEARING_CODES),
            uniform_concentration_rate: RecordValue::<f64>::new(&parts[1], "%", 1f64),
            non_uniform_concentration_code: CodeRecord::new(
                &parts[2],
                &NON_UNIFORM_CONCENTRATION_CODES,
            ),
            ship_relative_position_code: CodeRecord::new(&parts[3], &SHIP_RELATIVE_POSITION_CODES),
            ship_penatrability_code: CodeRecord::new(&parts[4], &SHIP_PENETRABILITY_CODES),
            ice_trend_code: CodeRecord::new(&parts[5], &ICE_TREND_CODES),
            development_code: CodeRecord::new(&parts[6], &DEVELOPMENT_CODES),
            growler_bergy_bit_presence_code: CodeRecord::new(
                &parts[7],
                &GROWLER_BERGY_BIT_PRESENCE_CODE,
            ),
            growler_bergy_bit_quantity: RecordValue::<isize>::new(&parts[8], "", 1isize),
            iceberg_quantity: RecordValue::<isize>::new(&parts[9], "", 1isize),
            quality_code: CodeRecord::new(&parts[10], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct WG1 {
    edge_bearing_code: CodeRecord,
    edge_distance_dimension: Option<RecordValue<f64>>,
    edge_orientation_code: CodeRecord,
    formation_type_code: CodeRecord,
    navigation_effect_code: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for WG1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(WG1 {
            edge_bearing_code: CodeRecord::new(&parts[0], &EDGE_BEARING_CODES),
            edge_distance_dimension: RecordValue::<f64>::new(&parts[1], "km", 10f64),
            edge_orientation_code: CodeRecord::new(&parts[2], &EDGE_ORIENTATION_CODES),
            formation_type_code: CodeRecord::new(&parts[3], &FORMATION_TYPE_CODES),
            navigation_effect_code: CodeRecord::new(&parts[4], &NAVIGATION_EFFECT_CODES),
            quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct WJ1 {
    ice_thickness: Option<RecordValue<f64>>,
    discharge_rate: Option<RecordValue<f64>>,
    primary_ice_phenomenon: CodeRecord,
    secondary_ice_phenomenon: CodeRecord,
    stage_height: Option<RecordValue<f64>>,
    under_ice_slush_condition: CodeRecord,
    water_level: CodeRecord,
}
impl FromStr for WJ1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(WJ1 {
            ice_thickness: RecordValue::<f64>::new(&parts[0], "cm", 1f64),
            discharge_rate: RecordValue::<f64>::new(&parts[1], "m^3/s", 1f64),
            primary_ice_phenomenon: CodeRecord::new(&parts[2], &ICE_PHENOMENA_CODES),
            secondary_ice_phenomenon: CodeRecord::new(&parts[3], &ICE_PHENOMENA_CODES),
            stage_height: RecordValue::<f64>::new(&parts[4], "cm", 1f64),
            under_ice_slush_condition: CodeRecord::new(&parts[5], &UNDER_ICE_SLUSH_CONDITION_CODES),
            water_level: CodeRecord::new(&parts[6], &WATER_LEVEL_CODES),
        })
    }
}
