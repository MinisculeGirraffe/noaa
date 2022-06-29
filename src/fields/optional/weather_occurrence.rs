use std::str::FromStr;

use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{Serialize};
use serde_with::DeserializeFromStr;

pub static SOURCE_ELEMENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "AU" => "sourced from automated ASOS/AWOS sensors",
    "AW" => "sourced from automated sensors",
    "MW" => "sourced from manually reported present weather",
};

pub static WEATHER_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    "01" => "Fog, ice fog or freezing fog (may include heavy fog)",
    "02" => "Heavy fog or heavy freezing fog (not always distinguished from fog)",
    "03" => "Thunder",
    "04" => "Ice pellets, sleet, snow pellets or small hail",
    "05" => "Hail (may include small hail)",
    "06" => "Glaze or rime",
    "07" => "Dust, volcanic ash, blowing dust, blowing sand or blowing obstruction",
    "08" => "Smoke or haze",
    "09" => "Blowing or drifting snow",
    "10" => "Tornado, water spout or funnel cloud",
    "11" => "High or damaging winds",
    "12" => "Blowing spray",
    "13" => "Mist",
    "14" => "Drizzle",
    "15" => "Freezing drizzle",
    "16" => "Rain",
    "17" => "Freezing rain",
    "18" => "Snow, snow pellets, snow grains or ice crystals",
    "19" => "Unknown precipitation",
    "21" => "Ground fog",
    "22" => "Ice fog or freezing fog",
};
pub static WEATHER_TYPE_ABBREVIATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "FG" => "Fog, ice fog or freezing fog (may include heavy fog)",
    "FG+" => "Heavy fog or heavy freezing fog (not always distinguished from fog)",
    "TS" => "Thunder",
    "PL" => "Ice pellets, sleet, snow pellets or small hail",
    "GR" => "Hail (may include small hail)",
    "GL" => "Glaze or rime",
    "DU" => "Dust, volcanic ash, blowing dust, blowing sand or blowing obstruction",
    "HZ" => "Smoke or haze",
    "BLSN" => "Blowing or drifting snow",
    "FC" => "Tornado, water spout or funnel cloud",
    "WIND" => "High or damaging winds",
    "BLPY" => "Blowing spray",
    "BR" => "Mist",
    "DZ" => "Drizzle",
    "FZDZ" => "Freezing drizzle",
    "RA" => "Rain",
    "FZRA" => "Freezing rain",
    "SN" => "Snow, snow pellets, snow grains or ice crystals",
    "UP" => "Unknown precipitation",
    "MIFG" => "Ground fog",
    "FZFG" => "Ice fog or freezing fog",
};

pub static INTENSITY_PROXIMITY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Not Reported",
    "1" => "Light (-)",
    "2" => "Moderate or Not Reported (no entry in original observation)",
    "3" => "Heavy (+)",
    "4" => "Vicinity (VC)",
    "9" => "Missing",
};

pub static DESCRIPTOR_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "No Descriptor",
    "1" => "Shallow (MI)",
    "2" => "Partial (PR)",
    "3" => "Patches (BC)",
    "4" => "Low Drifting (DR)",
    "5" => "Blowing (BL)",
    "6" => "Shower(s) (SH)",
    "7" => "Thunderstorm (TS)",
    "8" => "Freezing (FZ)",
    "9" => "Missing",
};

pub static PRECIPITATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No Precipitation",
    "01" => "Drizzle (DZ)",
    "02" => "Rain (RA)",
    "03" => "Snow (SN)",
    "04" => "Snow Grains (SG)",
    "05" => "Ice Crystals (IC)",
    "06" => "Ice Pellets (PL)",
    "07" => "Hail (GR)",
    "08" => "Small Hail and/or Snow Pellets (GS)",
    "09" => "Unknown Precipitation (UP)",
    "99" => "Missing",
};

pub static OBSCURATION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "No Obscuration",
    "1" => "Mist (BR)",
    "2" => "Fog (FG)",
    "3" => "Smoke (FU)",
    "4" => "Volcanic Ash (VA)",
    "5" => "Widespread Dust (DU)",
    "6" => "Sand (SA)",
    "7" => "Haze (HZ)",
    "8" => "Spray (PY)",
    "9" => "Missing",
};

pub static OTHER_WEATHER_PHENOMENA_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "None Reported",
    "1" => "Well-Developed Dust/Sand Whirls (PO)",
    "2" => "Squalls (SQ)",
    "3" => "Funnel Cloud, Tornado, Waterspout (FC)",
    "4" => "Sandstorm (SS)",
    "5" => "Duststorm (DS)",
    "9" => "Missing",
};

pub static COMBINATION_INDICATOR_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Not part of combined weather elements",
    "2" => "Beginning element of combined weather elements",
    "3" => "Combined with previous weather element to form a single weather report",
    "9" => "Missing",
};

pub static AUTOMATED_ATMOSPHERIC_CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No significant weather observed",
    "01" => "Clouds generally dissolving or becoming less developed",
    "02" => "State of sky on the whole unchanged during the past hour",
    "03" => "Clouds generally forming or developing during the past hour",
    "04" => "Haze, smoke, or dust in suspension in the air, visibility equal to or greater than 1km",
    "05" => "Smoke",
    "07" => "Dust or sand raised by wind at or near the station at the time of observation, but no well-developed dust irl(s) whirls(s) or sand whirl(s), and no duststorm or sandstorm seen or, in the case of ships, blowing spray at the station",
    "10" => "Mist",
    "11" => "Diamond dust",
    "12" => "Distant lightning",
    "18" => "Squalls",
    "20" => "Fog",
    "21" => "Precipitation",
    "22" => "Drizzle (not freezing) or snow grains",
    "23" => "Rain (not freezing)",
    "24" => "Snow",
    "25" => "Freezing drizzle or freezing rain",
    "26" => "Thunderstorm (with or without precipitation)",
    "27" => "Blowing or drifting snow or sand",
    "28" => "Blowing or drifting snow or sand, visibility equal to or greater than 1 km",
    "29" => "Blowing or drifting snow or sand, visibility less than 1 km",
    "30" => "Fog",
    "31" => "Fog or ice fog in patches",
    "32" => "Fog or ice fog, has become thinner during the past hour",
    "33" => "Fog or ice fog, no appreciable change during the past hour",
    "34" => "Fog or ice fog, has begun or become thicker during the past hour",
    "35" => "Fog, depositing rime",
    "40" => "Precipitation",
    "41" => "Precipitation, slight or moderate",
    "42" => "Precipitation, heavy",
    "43" => "Liquid precipitation, slight or moderate",
    "44" => "Liquid precipitation, heavy",
    "45" => "Solid precipitation, slight or moderate",
    "46" => "Solid precipitation, heavy",
    "47" => "Freezing precipitation, slight or moderate",
    "48" => "Freezing precipitation, heavy",
    "50" => "Drizzle",
    "51" => "Drizzle, not freezing, slight",
    "52" => "Drizzle, not freezing, moderate",
    "53" => "Drizzle, not freezing, heavy",
    "54" => "Drizzle, freezing, slight",
    "55" => "Drizzle, freezing, moderate",
    "56" => "Drizzle, freezing, heavy",
    "57" => "Drizzle and rain, slight",
    "58" => "Drizzle and rain, moderate or heavy",
    "60" => "Rain",
    "61" => "Rain, not freezing, slight",
    "62" => "Rain, not freezing, moderate",
    "63" => "Rain, not freezing, heavy",
    "64" => "Rain, freezing, slight",
    "65" => "Rain, freezing, moderate",
    "66" => "Rain, freezing, heavy",
    "67" => "Rain or drizzle and snow, slight",
    "68" => "Rain or drizzle and snow, moderate or heavy",
    "70" => "Snow",
    "71" => "Snow, slight",
    "72" => "Snow, moderate",
    "73" => "Snow, heavy",
    "74" => "Ice pellets, slight",
    "75" => "Ice pellets, moderate",
    "76" => "Ice pellets, heavy",
    "77" => "Snow grains",
    "78" => "Ice crystals",
    "80" => "Showers or intermittent precipitation",
    "81" => "Rain showers or intermittent rain, slight",
    "82" => "Rain showers or intermittent rain, moderate",
    "83" => "Rain showers or intermittent rain, heavy",
    "84" => "Rain showers or intermittent rain, violent",
    "85" => "Snow showers or intermittent snow, slight",
    "86" => "Snow showers or intermittent snow, moderate",
    "87" => "Snow showers or intermittent snow, heavy",
    "89" => "Hail",
    "90" => "Thunderstorm",
    "91" => "Thunderstorm, slight or moderate, with no precipitation",
    "92" => "Thunderstorm, slight or moderate, with rain showers and/or snow showers",
    "93" => "Thunderstorm, slight or moderate, with hail",
    "94" => "Thunderstorm, heavy, with no precipitation",
    "95" => "Thunderstorm, heavy, with rain showers and/or snow",
    "96" => "Thunderstorm, heavy, with hail",
    "99" => "Tornado",
};

pub static ATMOSPHERIC_CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "none to report",
    "01" => "fog",
    "02" => "fog reducing visibility to ¼ mile or less",
    "03" => "thunder",
    "04" => "ice pellets",
    "05" => "hail",
    "06" => "glaze or rime",
    "07" => "blowing dust or sand, visibility ½ mile or less",
    "08" => "smoke or haze",
    "09" => "blowing snow",
    "10" => "tornado",
    "11" => "high or damaging winds",
    "99" => "missing",
};

pub static MANUAL_ATMOSPHERIC_CONDITION_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Cloud covering 1/2 or less of the sky throughout the appropriate period",
    "1" => "Cloud covering more than ½ of the sky during part of the appropriate period and covering ½ or less during part of the period",
    "2" => "Cloud covering more than 1/2 of the sky throughout the appropriate period",
    "3" => "Sandstorm, duststorm or blowing snow",
    "4" => "Fog or ice fog or thick haze",
    "5" => "Drizzle",
    "6" => "Rain",
    "7" => "Snow, or rain and snow mixed",
    "8" => "Shower(s)",
    "9" => "Thunderstorm(s) with or without precipitation",
    "00" => "Cloud development not observed or not observable",
    "01" => "Clouds generally dissolving or becoming less developed",
    "02" => "State of sky on the whole unchanged",
    "03" => "Clouds generally forming or developing",
    "04" => "Visibility reduced by smoke, e.g. veldt or forest fires, industrial smoke or volcanic ashes",
    "05" => "Haze",
    "06" => "Widespread dust in suspension in the air, not raised by wind at or near the station at the time of observation",
    "07" => "Dust or sand raised by wind at or near the station at the time of observation, but no well-developed dust whirl(s) sand whirl(s), and no duststorm or sandstorm seen or, in the case of ships, blowing spray at the station",
    "08" => "Well developed dust whirl(s) or sand whirl(s) seen at or near the station during the preceding hour or at the time of observation, but no duststorm or sandstorm",
    "09" => "Duststorm or sandstorm within sight at the time of observation, or at the station during the preceding hour",
    "10" => "Mist",
    "11" => "Patches of shallow fog or ice fog at the station, whether on land or sea, not deeper than about 2 meters on land or 10 meters at sea",
    "12" => "More or less continuous shallow fog or ice fog at the station, whether on land or sea, not deeper than about 2 meters on land or 10 meters at sea",
    "13" => "Lightning visible, no thunder heard",
    "14" => "Precipitation within sight, not reaching the ground or the surface of the sea",
    "15" => "Precipitation within sight, reaching the ground or the surface of the sea, but distant, i.e., estimated to be more than 5 km from the station",
    "16" => "Precipitation within sight, reaching the ground or the surface of the sea, near to, but not at the station",
    "17" => "Thunderstorm, but no precipitation at the time of observation",
    "18" => "Squalls at or within sight of the station during the preceding hour or at the time of observation",
    "19" => "Funnel cloud(s) (Tornado cloud or waterspout) at or within sight of the station during the preceding hour or at the time of observation,",
    "20" => "Drizzle (not freezing) or snow grains not falling as shower(s)",
    "21" => "Rain (not freezing) not falling as shower(s)",
    "22" => "Snow not falling as shower(s)",
    "23" => "Rain and snow or ice pellets not falling as shower(s)",
    "24" => "Freezing drizzle or freezing rain not falling as shower(s)",
    "25" => "Shower(s) of rain",
    "26" => "Shower(s) of snow or of rain and snow",
    "27" => "Shower(s) of hail (Hail, small hail, snow pellets), or rain and hail",
    "28" => "Fog or ice fog",
    "29" => "Thunderstorm (with or without precipitation),",
    "30" => "Slight or moderate duststorm or sandstorm has decreased during the preceding hour",
    "31" => "Slight or moderate duststorm or sandstorm no appreciable change during the preceding hour",
    "32" => "Slight or moderate duststorm or sandstorm has begun or has increased during the preceding hour",
    "33" => "Severe duststorm or sandstorm has decreased during the preceding hour",
    "34" => "Severe duststorm or sandstorm no appreciable change during the preceding hour",
    "35" => "Severe duststorm or sandstorm has begun or has increased during the preceding hour",
    "36" => "Slight or moderate drifting snow generally low (below eye level)",
    "37" => "Heavy drifting snow generally low (below eye level)",
    "38" => "Slight or moderate blowing snow generally high (above eye level)",
    "39" => "Heavy blowing snow generally high (above eye level)",
    "40" => "Fog or ice fog at a distance at the time of observation, but not at the station during the preceding hour, the fog or ice fog extending to a level above that of the observer",
    "41" => "Fog or ice fog in patches",
    "42" => "Fog or ice fog, sky visible, has become thinner during the preceding hour",
    "43" => "Fog or ice fog, sky invisible, has become thinner during the preceding hour",
    "44" => "Fog or ice fog, sky visible, no appreciable change during the preceding hour",
    "45" => "Fog or ice fog, sky invisible, no appreciable change during the preceding hour",
    "46" => "Fog or ice fog, sky visible, has begun or has become thicker during the preceding hour",
    "47" => "Fog or ice fog, sky invisible, has begun or has become thicker during the preceding hour",
    "48" => "Fog, depositing rime, sky visible",
    "49" => "Fog, depositing rime, sky invisible",
    "50" => "Drizzle, not freezing, intermittent, slight at time of observation",
    "51" => "Drizzle, not freezing, continuous, slight at time of observation",
    "52" => "Drizzle, not freezing, intermittent, moderate at time of observation",
    "53" => "Drizzle, not freezing, continuous, moderate at time of observation",
    "54" => "Drizzle, not freezing, intermittent, heavy (dense) at time of observation",
    "55" => "Drizzle, not freezing, continuous, heavy (dense) at time of observation",
    "56" => "Drizzle, freezing, slight",
    "57" => "Drizzle, freezing, moderate or heavy (dense)",
    "58" => "Drizzle and rain, slight",
    "59" => "Drizzle and rain, moderate or heavy",
    "60" => "Rain, not freezing, intermittent, slight at time of observation",
    "61" => "Rain, not freezing, continuous, slight at time of observation",
    "62" => "Rain, not freezing, intermittent, moderate at time of observation",
    "63" => "Rain, not freezing, continuous, moderate at time of observation",
    "64" => "Rain, not freezing, intermittent, heavy at time of observation",
    "65" => "Rain, not freezing, continuous, heavy at time of observation",
    "66" => "Rain, freezing, slight",
    "67" => "Rain, freezing, moderate or heavy",
    "68" => "Rain or drizzle and snow, slight",
    "69" => "Rain or drizzle and snow, moderate or heavy",
    "70" => "Intermittent fall of snowflakes, slight at time of observation",
    "71" => "Continuous fall of snowflakes, slight at time of observation",
    "72" => "Intermittent fall of snowflakes, moderate at time of observation",
    "73" => "Continuous fall of snowflakes, moderate at time of observation",
    "74" => "Intermittent fall of snowflakes, heavy at time of observation",
    "75" => "Continuous fall of snowflakes, heavy at time of observation",
    "76" => "Diamond dust (with or without fog)",
    "77" => "Snow grains (with or without fog)",
    "78" => "Isolated star-like snow crystals (with or without fog)",
    "79" => "Ice pellets",
    "80" => "Rain shower(s), slight",
    "81" => "Rain shower(s), moderate or heavy",
    "82" => "Rain shower(s), violent",
    "83" => "Shower(s) of rain and snow mixed, slight",
    "84" => "Shower(s) of rain and snow mixed, moderate or heavy",
    "85" => "Show shower(s), slight",
    "86" => "Snow shower(s), moderate or heavy",
    "87" => "Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed, slight",
    "88" => "Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed, moderate or heavy",
    "89" => "Shower(s) of hail (hail, small hail, snow pellets), with or without rain or rain and snow mixed, not associated with thunder, slight",
    "90" => "Shower(s) of hail (hail, small hail, snow pellets), with or without rain or rain and snow mixed, not associated with thunder, moderate or heavy",
    "91" => "Slight rain at time of observation, thunderstorm during the preceding hour but not at time of observation",
    "92" => "Moderate or heavy rain at time of observation, thunderstorm during the preceding hour but not at time of observation",
    "93" => "Slight snow, or rain and snow mixed or hail (Hail, small hail, snow pellets), at time of observation, thunderstorm during the preceding hour but not at time of observation",
    "94" => "Moderate or heavy snow, or rain and snow mixed or hail(Hail, small hail, snow pellets) at time of observation, thunderstorm during the preceding hour but not at time of observation",
    "95" => "Thunderstorm, slight or moderate, without hail (Hail, small hail, snow pellets), but with rain and/or snow at time of observation, thunderstorm at time of observation",
    "96" => "Thunderstorm, slight or moderate, with hail (hail, small hail, snow pellets) at time of observation, thunderstorm at time of observation",
    "97" => "Thunderstorm, heavy, without hail (Hail, small hail, snow pellets), but with rain and/or snow at time of observation, thunderstorm at time of observation",
    "98" => "Thunderstorm combined with duststorm or sandstorm at time of observation, thunderstorm at time of observation",
    "99" => "Thunderstorm, heavy, with hail (Hail, small hail, snow pellets) at time of observation, thunderstorm at time of observation",
};
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
// AT1 – AT8
pub struct ATX {
    source_element: CodeRecord,
    weather_type: CodeRecord,
    weather_type_abbreviation: CodeRecord,
    quality_code: CodeRecord,
}
impl FromStr for ATX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(ATX {
            source_element: CodeRecord::new(&parts[0], &SOURCE_ELEMENTS),
            weather_type: CodeRecord::new(&parts[1], &WEATHER_TYPES),
            weather_type_abbreviation: CodeRecord::new(&parts[2], &WEATHER_TYPE_ABBREVIATIONS),
            quality_code: CodeRecord::new(&parts[3], &&QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//AU1 - AU9
pub struct AUX {
    intensity_code: CodeRecord,
    descriptor_code: CodeRecord,
    precipitation_code: CodeRecord,
    obscuration_code: CodeRecord,
    weather_phenomena_code: CodeRecord,
    combination_indicator_code: CodeRecord,
    quality_code: CodeRecord,
}

impl FromStr for AUX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(AUX {
            intensity_code: CodeRecord::new(&parts[0], &INTENSITY_PROXIMITY_CODES),
            descriptor_code: CodeRecord::new(&parts[1], &DESCRIPTOR_CODES),
            precipitation_code: CodeRecord::new(&parts[2], &PRECIPITATION_CODES),
            obscuration_code: CodeRecord::new(&parts[3], &OBSCURATION_CODES),
            weather_phenomena_code: CodeRecord::new(&parts[4], &OTHER_WEATHER_PHENOMENA_CODES),
            combination_indicator_code: CodeRecord::new(&parts[5], &COMBINATION_INDICATOR_CODES),
            quality_code: CodeRecord::new(&parts[6], &&QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
// AW1 - AW4
pub struct AWX {
    atmospheric_condition_code: CodeRecord,
}

impl FromStr for AWX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;
        Ok(AWX {
            atmospheric_condition_code: CodeRecord::new(
                &parts[0],
                &AUTOMATED_ATMOSPHERIC_CONDITION_CODES,
            ),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
// AX1 - AX6
pub struct AXX {
    atmospheric_condition_code: CodeRecord,
    atmospheric_condition_quality_code: CodeRecord,
    period_quantity: Option<RecordValue<i8>>,
    period_quality_code: CodeRecord,
}

impl FromStr for AXX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(AXX {
            atmospheric_condition_code: CodeRecord::new(&parts[0], &ATMOSPHERIC_CONDITION_CODES),
            atmospheric_condition_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            period_quantity: RecordValue::new(&parts[2], "h", 1),
            period_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//AY1 - AY2
pub struct AYX {
    manual_atmospheric_condition_code: CodeRecord,
    manual_atmospheric_condition_quality_code: CodeRecord,
    period_quantity: Option<RecordValue<i8>>,
    period_quality_code: CodeRecord,
}

impl FromStr for AYX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(AYX {
            manual_atmospheric_condition_code: CodeRecord::new(
                &parts[0],
                &MANUAL_ATMOSPHERIC_CONDITION_CODES,
            ),
            manual_atmospheric_condition_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            period_quantity: RecordValue::new(&parts[2], "h", 1),
            period_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

//AZ1 - AZ2
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct AZX {
    automated_atmospheric_condition_code: CodeRecord,
    automated_atmospheric_condition_quality_code: CodeRecord,
    period_quantity: Option<RecordValue<i8>>,
    period_quality_code: CodeRecord,
}

impl FromStr for AZX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(AZX {
            automated_atmospheric_condition_code: CodeRecord::new(
                &parts[0],
                &AUTOMATED_ATMOSPHERIC_CONDITION_CODES,
            ),
            automated_atmospheric_condition_quality_code: CodeRecord::new(
                &parts[1],
                &QUALITY_CODES,
            ),
            period_quantity: RecordValue::new(&parts[2], "h", 1),
            period_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
        })
    }
}

//MW1-7 
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct MWX {
    manual_atmospheric_condition_code: CodeRecord,
    manual_atmospheric_condition_quality_code: CodeRecord,
}
impl FromStr for MWX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s)?;

        Ok(MWX {
            manual_atmospheric_condition_code: CodeRecord::new(
                &parts[0],
                &MANUAL_ATMOSPHERIC_CONDITION_CODES,
            ),
            manual_atmospheric_condition_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
