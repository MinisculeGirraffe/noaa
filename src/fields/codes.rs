use phf::phf_map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CodeRecord {
    pub value: String,
    pub description: String,
}

impl CodeRecord {
    pub fn new(value: &str, map: &phf::Map<&'static str, &'static str>) -> CodeRecord {
        //trim whitespace from value

        CodeRecord {
            value: value.to_string(),
            description: map.get(value.trim()).unwrap().to_string(),
        }
    }
}

pub static BOOL_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "N" => "No",
    "Y" => "Yes",
    "9" => "Missing",
};

pub static QUALITY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Passed gross limits check",
    "1" => "Passed all quality control checks",
    "2" => "Suspect",
    "3" => "Erroneous",
    "4" => "Passed gross limits check, data originate from an NCEI data source",
    "5" => "Passed all quality control checks, data originate from an NCEI data source",
    "6" => "Suspect, data originate from an NCEI data source",
    "7" => "Erroneous, data originate from an NCEI data source",
    "9" => "Passed gross limits check if element is present",
};
pub static DL_QUALITY_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Passed all quality control checks",
    "3" => "Failed all quality control checks",
    "9" => "Missing",
};

pub static DL_QUALITY_FLAG: phf::Map<&'static str, &'static str> = phf_map! {
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
pub static _SOURCE_FLAG: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "USAF SURFACE HOURLY observation, candidate for merge with NCEI SURFACE HOURLY (not yet merged, element cross-checks)",
    "2" => "NCEI SURFACE HOURLY observation, candidate for merge with USAF SURFACE HOURLY (not yet merged, failed element cross-checks)",
    "3" => "USAF SURFACE HOURLY/NCEI SURFACE HOURLY merged observation",
    "4" => "USAF SURFACE HOURLY observation",
    "5" => "NCEI SURFACE HOURLY observation",
    "6" => "ASOS/AWOS observation from NCEI",
    "7" => "ASOS/AWOS observation merged with USAF SURFACE HOURLY observation",
    "8" => "MAPSO observation (NCEI)",
    "A" => "USAF SURFACE HOURLY/NCEI HOURLY PRECIPITATION merged observation, candidate for merge with NCEI SURFACE HOURLY (not yet merged, failed element cross-checks)",
    "B" => "NCEI SURFACE HOURLY/NCEI HOURLY PRECIPITATION merged observation, candidate for merge with USAF SURFACE HOURLY (not yet merged, failed element cross-checks)",
    "C" => "USAF SURFACE HOURLY/NCEI SURFACE HOURLY/NCEI HOURLY PRECIPITATION merged observation",
    "D" => "USAF SURFACE HOURLY/NCEI HOURLY PRECIPITATION merged observation",
    "E" => "NCEI SURFACE HOURLY/NCEI HOURLY PRECIPITATION merged observation",
    "F" => "Form OMR/1001 - Weather Bureau city office (keyed data)",
    "G" => "SAO surface airways observation, pre-1949 (keyed data)",
    "H" => "SAO surface airways observation, 1965-1981 format/period (keyed data)",
    "I" => "Climate Reference Network observation",
    "J" => "Cooperative Network observation",
    "K" => "Radiation Network observation",
    "L" => "Data from Climate Data Modernization Program (CDMP) data source",
    "M" => "Data from National Renewable Energy Laboratory (NREL) data source",
    "N" => "NCAR / NCEI cooperative effort (various national datasets)",
    "O" => "Summary observation created by NCEI using hourly observations that may not share the same data source flag.",
    "P" => "Data value not originally flagged as suspect, but replaced by validator",
    "R" => "Data value replaced with value computed by NCEI software",
    "U" => "Data value replaced with edited value",
    "9" => "Missing",
};

pub static _REPORT_TYPE_CODE: phf::Map<&'static str, &'static str> = phf_map! {

 "AERO" => "Aerological report",
 "AUST" => "Dataset from Australia",
 "AUTO" => "Report from an automatic station",
 "BOGUS" => "Bogus report",
 "BRAZ" => "Dataset from Brazil",
 "COOPD" => "US Cooperative Network summary of day report",
 "COOPS" => "US Cooperative Network soil temperature report",
 "CRB" => "Climate Reference Book data from CDMP",
 "CRN05" => "Climate Reference Network report, with 5-minute reporting interval",
 "CRN15" => "Climate Reference Network report, with 15-minute reporting interval",
 "FM-12" => "SYNOP Report of surface observation form a fixed land station",
 "FM-13" => "SHIP Report of surface observation from a sea station",
 "FM-14" => "SYNOP MOBIL Report of surface observation from a mobile land station",
 "FM-15" => "METAR Aviation routine weather report",
 "FM-16" => "SPECI Aviation selected special weather report",
 "FM-18" => "BUOY Report of a buoy observation",
 "GREEN" => "Dataset from Greenland",
 "MESOH" => "Hydrological observations from MESONET operated civilian or government agency",
 "MESOS" => "MESONET operated civilian or government agency",
 "MESOW" => "Snow observations from MESONET operated civilian or government agency",
 "MEXIC" => "Dataset from Mexico",
 "NSRDB" => "National Solar Radiation Data Base",
 "PCP15" => "US 15-minute precipitation network report",
 "PCP60" => "US 60-minute precipitation network report",
 "S-S-A" => "Synoptic, airways, and auto merged report",
 "SA-AU" => "Airways and auto merged report",
 "SAO" => "Airways report (includes record specials)",
 "SAOSP" => "Airways special report (excluding record specials)",
 "SHEF" => "Standard Hydrologic Exchange Format",
 "SMARS" => "Supplementary airways station report",
 "SOD" => "Summary of day report from U.S. ASOS or AWOS station",
 "SOM" => "Summary of month report from U.S. ASOS or AWOS station",
 "SURF" => "Surface Radiation Network report",
 "SY-AE" => "Synoptic and aero merged report",
 "SY-AU" => "Synoptic and auto merged report",
 "SY-MT" => "Synoptic and METAR merged report",
 "SY-SA" => "Synoptic and airways merged report",
 "WBO" => "Weather Bureau Office",
 "WNO" => "Washington Naval Observatory",
 "99999" => "Missing",
};

