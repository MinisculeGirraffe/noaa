use std::str::FromStr;

use crate::fields::codes::{CodeRecord, QUALITY_CODES};
use crate::model::RecordValue;
use crate::util::get_parts;
use phf::phf_map;
use serde::{ Serialize};
use serde_with::DeserializeFromStr;

pub static COVERAGE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "None, SKC or CLR",
    "01" => "One okta - 1/10 or less but not zero",
    "02" => "Two oktas - 2/10 - 3/10, or FEW",
    "03" => "Three oktas - 4/10",
    "04" => "Four oktas - 5/10, or SCT",
    "05" => "Five oktas - 6/10",
    "06" => "Six oktas - 7/10 - 8/10",
    "07" => "Seven oktas - 9/10 or more but not 10/10, or BKN",
    "08" => "Eight oktas - 10/10, or OVC",
    "09" => "Sky obscured, or cloud amount cannot be estimated",
    "10" => "Partial Obscuration",
    "11" => "Thin Scattered",
    "12" => "Scattered",
    "13" => "Dark Scattered",
    "14" => "Thin Broken",
    "15" => "Broken",
    "16" => "Dark Broken",
    "17" => "Thin Overcast",
    "18" => "Overcast",
    "19" => "Dark overcast",
    "99" => "Missing",
    "0" => "Clear - No coverage",
    "1" => "FEW - 2/8 or less coverage (not including zero)",
    "2" => "SCATTERED - 3/8-4/8 coverage",
    "3" => "BROKEN - 5/8-7/8 coverage",
    "4" => "OVERCAST - 8/8 coverage",
    "5" => "OBSCURED",
    "6" => "PARTIALLY OBSCURED",
    "9" => "MISSING",
};

pub static CLOUD_TYPE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Cirrus (Ci)",
    "01" => "Cirrocumulus (Cc)",
    "02" => "Cirrostratus (Cs)",
    "03" => "Altocumulus (Ac)",
    "04" => "Altostratus (As)",
    "05" => "Nimbostratus (Ns)",
    "06" => "Stratocumulus (Sc)",
    "07" => "Stratus (St)",
    "08" => "Cumulus (Cu)",
    "09" => "Cumulonimbus (Cb)",
    "10" => "Cloud not visible owing to darkness, fog, duststorm, sandstorm, or other analogous phenonomena/sky obcured",
    "11" => "Not used",
    "12" => "Towering Cumulus (Tcu)",
    "13" => "Stratus fractus (Stfra)",
    "14" => "Stratocumulus Lenticular (Scsl)",
    "15" => "Cumulus Fractus (Cufra)",
    "16" => "Cumulonimbus Mammatus (Cbmam)",
    "17" => "Altocumulus Lenticular (Acsl)",
    "18" => "Altocumulus Castellanus (Accas)",
    "19" => "Altocumulus Mammatus (Acmam)",
    "20" => "Cirrocumulus Lenticular (Ccsl)",
    "21" => "Cirrus and/or Cirrocumulus",
    "22" => "jenkins-content-114Stratus and/or Fracto-stratus",
    "23" => "Cumulus and/or Fracto-cumulus",
    "99" => "Missing",
};

pub static CLOUD_ATTRIBURE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "None",
    "1" => "ACSL (Altocumulus Standing Lenticular)",
    "2" => "ACCAS (Altocumulus Castelanus)",
    "3" => "TCU (Towering Cumulus)",
    "4" => "MDT CU (Moderate Cumulus)",
    "5" => "CB/CB MAM DISTANT (Cumulonimbus or Cumulonimbus Mammatus in the distance)",
    "6" => "CB/CBMAM (Cumulonimbus or Cumulonimbus Mammatus within 20 nautical miles)",
    "7" => "Unknown",
    "9" => "missing",
};

pub static VERTICAL_DATUM_ATTRIBUTE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "AGL" => "Above Ground Level",
    "ALAT" => "Approximate lowest astronomical tide",
    "AP" => "Apparent",
    "CFB" => "Crest of first berm",
    "CRD" => "Columbia River datum",
    "ESLW" => "Equatorial Spring low water",
    "GCLWD" => "Gulf Coast low water datum",
    "HAT" => "Highest astronomical tide",
    "HHW" => "Higher high water",
    "HTWW" => "High tide wave wash",
    "HW" => "High water",
    "HWFC" => "High water full and change",
    "IND" => "Indefinite",
    "ISLW" => "Indian Spring low water",
    "LAT" => "Lowest astronomical tide",
    "LLW" => "Lowest low water",
    "LNLW" => "Lowest normal low water",
    "LRLW" => "Lower low water",
    "LSD" => "Land survey datum",
    "LW" => "Low water",
    "LWD" => "Low water datum",
    "LWFC" => "Low water full and charge",
    "MHHW" => "Mean higher high water",
    "MHLW" => "Mean higher low water",
    "MHW" => "Mean high water",
    "MHWN" => "Mean high water neap",
    "MHWS" => "Mean high water spring",
    "MLHW" => "Mean lower high water",
    "MLLW" => "Mean lower low water",
    "MLLWS" => "Mean lower low water springs",
    "MLWN" => "Mean low water neap",
    "MLW" => "Mean low water",
    "MLWS" => "Mean low water spring",
    "MSL" => "Mean sea level",
    "MTL" => "Mean tide level",
    "NC" => "No correction",
    "NT" => "Neap tide",
    "ST" => "Spring tide",
    "SWA" => "Storm wave action",
    "TLLW" => "Tropic lower low water",
    "UD" => "Undetermined",
    "UK" => "Unknown",
    "WGS84E" => "WGS84 Ellispoid",
    "WGS84G" => "WGS84 GEOID",
    "999999" => "missing",
};

pub static LOW_CLOUD_GENUS_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No low clouds",
    "01" => "Cumulus humulis or Cumulus fractus other than of bad weather or both",
    "02" => "Cumulus mediocris or congestus, with or without Cumulus of species fractus or humulis or Stratocumulus all having bases at the same level",
    "03" => "Cumulonimbus calvus, with or without Cumulus, Stratocumulus or Stratus",
    "04" => "Stratocumulus cumulogenitus",
    "05" => "Stratocumulus other than Stratocumulus cumulogenitus",
    "06" => "Stratus nebulosus or Stratus fractus other than of bad weather, or both",
    "07" => "Stratus fractus or Cumulus fractus of bad weather, both (pannus) usually below Altostratus or Nimbostratus.",
    "08" => "Cumulus and Stratocumulus other than Stratocumulus cumulogenitus, with bases at different levels",
    "09" => "Cumulonimbus capillatus (often with an anvil), with or without Cumulonimbus calvus, Cumulus, Stratocumulus, Stratus or pannus",
    "99" => "Missing",
};

pub static MID_CLOUD_GENUS_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No middle clouds",
    "01" => "Altostratus translucidus",
    "02" => "Altostratus opacus or Nimbostratus",
    "03" => "Altocumulus translucidus at a single level",
    "04" => "Patches (often lenticulre) of Altocumulus translucidus, continually changing and occurring at one or more levels",
    "05" => "Altocumulus translucidus in bands, or one or more layers of Altocumulus translucidus or opacus, progressing invading the sky; these Altocumulus clouds generally thicken as a whole",
    "06" => "Altocumulus cumulogentis (or cumulonimbogentus)",
    "07" => "Altocumulus translucidus or opacus in two or more layers, or Altocumulus opacus in a single layer, not progressively invading the sky, or Altocumulus with Altostratus or Nimbostratus",
    "08" => "Altocumulus castellanus or floccus",
    "09" => "Altocumulus of a chaotic sky; generally at several levels",
    "99" => "Missing",
};

pub static HIGH_CLOUD_GENUS_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "No High Clouds",
    "01" => "Cirrus fibratus, sometimes uncinus, not progressively invading the sky",
    "02" => "Cirrus spissatus, in patches or entangled sheaves, which usually do not increase and sometimes seem to be the remains of the upper part of a Cumulonimbus; or Cirrus castellanus or floccus",
    "03" => "Cirrus spissatus cumulonimbogenitus",
    "04" => "Cirrus unicinus or fibratus, or both, progressively invading the sky; they generally thicken as a whole",
    "05" => "Cirrus (often in bands) and Cirrostratus, or Cirrostratus alone, progressively invading the sky; they generally thicken as a whole, but the continuous veil does not reach 45 degrees above the horizon",
    "06" => "Cirrus (often in bands) and Cirrostratus, or Cirrostratus alone, progressively invading the sky; they generally thicken as a whole; the continuous veil extends more than 45 degrees above the horizon, without the sky being totally covered.",
    "07" => "Cirrostratus covering the whole sky",
    "08" => "Cirrostratus not progressively invading the sky and not entirely covering it",
    "09" => "Cirrocumulus alone, or Cirrocumulus predominant among the High clouds",
    "99" => "Missing",
};

pub static TOP_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Isolated cloud or fragments of clouds",
    "01" => "Continuous flat tops",
    "02" => "Broken cloud - small breaks, flat tops",
    "03" => "Broken cloud - large breaks, flat tops",
    "04" => "Continuous cloud, undulation tops",
    "05" => "Broken cloud - small breaks, undulating tops",
    "06" => "Broken cloud - large breaks, undulating tops",
    "07" => "Continuous or almost continuous with towering clouds above the top of the layer",
    "08" => "Groups of waves with towering clouds above the top of the layer",
    "09" => "Two of more layers at different levels",
    "99" => "Missing",
};

pub static CHARACTERISTIC_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Variable height",
    "2" => "Variable amount",
    "3" => "Thin clouds",
    "4" => "Dark layer (reported in data prior to 1950)",
    "9" => "Missing",
};
pub static QUALITY_FLAG_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "Passed all quality control checks",
    "1" => "Did not pass all quality checks",
    "2" => "Did not pass all quality checks",
    "3" => "Did not pass all quality checks",
    "4" => "Did not pass all quality checks",
    "5" => "Did not pass all quality checks",
    "6" => "Did not pass all quality checks",
    "7" => "Did not pass all quality checks",
    "8" => "Did not pass all quality checks",
    "9" => "Did not pass all quality checks",
};
// TODO: Statically mapping codes to names is a bit of a hack.
// PHF maps are not really meant for this. 
// using lazystatic over a match statement is probably a better idea.
pub static DATA_FLAG_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "00" => "Untested (raw data)",
    "01" => "Passed one-component test; data fall within max-min limits of Kt, Kn, or Kd",
    "02" => "Passed two-component test; data fall within 0.03 of the Gompertz boundaries",
    "03" => "Passed three-component test; data come within + 0.03 of satisfying Kt = Kn + Kd",
    "04" => "Passed visual inspection: not used by SERI_QC1",
    "05" => "Failed visual inspection: not used by SERI_QC1",
    "06" => "Value estimated; passes all pertinent SERI_QC tests",
    "07" => "Failed one-component test; lower than allowed minimum",
    "08" => "Failed one-component test; higher than allowed maximum",
    "09" => "Passed three-component test but failed two-component test by 0.05",
    "10" => "Failed two- or three- component tests in one of four ways.",
    "11" => "Failed two- or three- component tests in one of four ways.",
    "12" => "Failed two- or three- component tests in one of four ways.",
    "13" => "Failed two- or three- component tests in one of four ways.",
    "14" => "Failed two- or three- component tests in one of four ways.",
    "15" => "Failed two- or three- component tests in one of four ways.",
    "16" => "Failed two- or three- component tests in one of four ways.",
    "17" => "Failed two- or three- component tests in one of four ways.",
    "18" => "Failed two- or three- component tests in one of four ways.",
    "19" => "Failed two- or three- component tests in one of four ways.",
    "20" => "Failed two- or three- component tests in one of four ways.",
    "21" => "Failed two- or three- component tests in one of four ways.",
    "22" => "Failed two- or three- component tests in one of four ways.",
    "23" => "Failed two- or three- component tests in one of four ways.",
    "24" => "Failed two- or three- component tests in one of four ways.",
    "25" => "Failed two- or three- component tests in one of four ways.",
    "26" => "Failed two- or three- component tests in one of four ways.",
    "27" => "Failed two- or three- component tests in one of four ways.",
    "28" => "Failed two- or three- component tests in one of four ways.",
    "29" => "Failed two- or three- component tests in one of four ways.",
    "30" => "Failed two- or three- component tests in one of four ways.",
    "31" => "Failed two- or three- component tests in one of four ways.",
    "32" => "Failed two- or three- component tests in one of four ways.",
    "33" => "Failed two- or three- component tests in one of four ways.",
    "34" => "Failed two- or three- component tests in one of four ways.",
    "35" => "Failed two- or three- component tests in one of four ways.",
    "36" => "Failed two- or three- component tests in one of four ways.",
    "37" => "Failed two- or three- component tests in one of four ways.",
    "38" => "Failed two- or three- component tests in one of four ways.",
    "39" => "Failed two- or three- component tests in one of four ways.",
    "40" => "Failed two- or three- component tests in one of four ways.",
    "41" => "Failed two- or three- component tests in one of four ways.",
    "42" => "Failed two- or three- component tests in one of four ways.",
    "43" => "Failed two- or three- component tests in one of four ways.",
    "44" => "Failed two- or three- component tests in one of four ways.",
    "45" => "Failed two- or three- component tests in one of four ways.",
    "46" => "Failed two- or three- component tests in one of four ways.",
    "47" => "Failed two- or three- component tests in one of four ways.",
    "48" => "Failed two- or three- component tests in one of four ways.",
    "49" => "Failed two- or three- component tests in one of four ways.",
    "50" => "Failed two- or three- component tests in one of four ways.",
    "51" => "Failed two- or three- component tests in one of four ways.",
    "52" => "Failed two- or three- component tests in one of four ways.",
    "53" => "Failed two- or three- component tests in one of four ways.",
    "54" => "Failed two- or three- component tests in one of four ways.",
    "55" => "Failed two- or three- component tests in one of four ways.",
    "56" => "Failed two- or three- component tests in one of four ways.",
    "57" => "Failed two- or three- component tests in one of four ways.",
    "58" => "Failed two- or three- component tests in one of four ways.",
    "59" => "Failed two- or three- component tests in one of four ways.",
    "60" => "Failed two- or three- component tests in one of four ways.",
    "61" => "Failed two- or three- component tests in one of four ways.",
    "62" => "Failed two- or three- component tests in one of four ways.",
    "63" => "Failed two- or three- component tests in one of four ways.",
    "64" => "Failed two- or three- component tests in one of four ways.",
    "65" => "Failed two- or three- component tests in one of four ways.",
    "66" => "Failed two- or three- component tests in one of four ways.",
    "67" => "Failed two- or three- component tests in one of four ways.",
    "68" => "Failed two- or three- component tests in one of four ways.",
    "69" => "Failed two- or three- component tests in one of four ways.",
    "70" => "Failed two- or three- component tests in one of four ways.",
    "71" => "Failed two- or three- component tests in one of four ways.",
    "72" => "Failed two- or three- component tests in one of four ways.",
    "73" => "Failed two- or three- component tests in one of four ways.",
    "74" => "Failed two- or three- component tests in one of four ways.",
    "75" => "Failed two- or three- component tests in one of four ways.",
    "76" => "Failed two- or three- component tests in one of four ways.",
    "77" => "Failed two- or three- component tests in one of four ways.",
    "78" => "Failed two- or three- component tests in one of four ways.",
    "79" => "Failed two- or three- component tests in one of four ways.",
    "80" => "Failed two- or three- component tests in one of four ways.",
    "81" => "Failed two- or three- component tests in one of four ways.",
    "82" => "Failed two- or three- component tests in one of four ways.",
    "83" => "Failed two- or three- component tests in one of four ways.",
    "84" => "Failed two- or three- component tests in one of four ways.",
    "85" => "Failed two- or three- component tests in one of four ways.",
    "86" => "Failed two- or three- component tests in one of four ways.",
    "87" => "Failed two- or three- component tests in one of four ways.",
    "88" => "Failed two- or three- component tests in one of four ways.",
    "89" => "Failed two- or three- component tests in one of four ways.",
    "90" => "Failed two- or three- component tests in one of four ways.",
    "91" => "Failed two- or three- component tests in one of four ways.",
    "92" => "Failed two- or three- component tests in one of four ways.",
    "93" => "Failed two- or three- component tests in one of four ways.",
    "94" => "Data fails into physically impossible region where Kn > Kt by K-space distances of 0.05 to 0.10.",
    "95" => "Data fails into physically impossible region where Kn > Kt by K-space distances of 0.10 to 0.15.",
    "96" => "Data fails into physically impossible region where Kn > Kt by K-space distances of 0.15 to 0.20.",
    "97" => "Data fails into physically impossible region where Kn > Kt by K-space distances of > 0.20.",
    "98" => "Not used",
    "99" => "Missing data",
};

pub static MOD_GLOBAL_HORIZONTAL_SOURCE_FLAGS: phf::Map<&'static str, &'static str> = phf_map! {
    "01" => "Value modeled from METSTAT model",
    "02" => "Value time-shifted from SUNY satellite model",
    "03" => "Value time-shifted from SUNY satellite model, adjusted to a minimum low-diffuse envelope",
    "99" => "Missing data",
};

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//GA1-6
pub struct GAX {
    coverage_code: CodeRecord,
    coverage_quality_code: CodeRecord,
    base_height: Option<RecordValue<i32>>,
    base_height_quality_code: CodeRecord,
    cloud_type_code: CodeRecord,
    cloud_type_quality_code: CodeRecord,
}

impl FromStr for GAX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GAX {
            coverage_code: CodeRecord::new(&parts[0], &COVERAGE_CODES),
            coverage_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            base_height: RecordValue::new(&parts[2], "meters", 1),
            base_height_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            cloud_type_code: CodeRecord::new(&parts[4], &CLOUD_TYPE_CODES),
            cloud_type_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//GD1-6
pub struct GDX {
    coverage_code: CodeRecord,
    coverage_code_2: CodeRecord,
    coverage_quality_code: CodeRecord,
    height_dimension: Option<RecordValue<i32>>,
    height_dimension_quality_code: CodeRecord,
    characteristic_code: CodeRecord,
}

impl FromStr for GDX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GDX {
            coverage_code: CodeRecord::new(&parts[0], &COVERAGE_CODES),
            coverage_code_2: CodeRecord::new(&parts[1], &COVERAGE_CODES),
            coverage_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            height_dimension: RecordValue::new(&parts[3], "meters", 1),
            height_dimension_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            characteristic_code: CodeRecord::new(&parts[5], &CHARACTERISTIC_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GE1 {
    connective_cloud_code: CodeRecord,
    vertical_datum_code: CodeRecord,
    base_height_ur: Option<RecordValue<i32>>,
    base_height_lr: Option<RecordValue<i32>>,
}
impl FromStr for GE1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GE1 {
            connective_cloud_code: CodeRecord::new(&parts[0], &CLOUD_ATTRIBURE_CODES),
            vertical_datum_code: CodeRecord::new(&parts[1], &VERTICAL_DATUM_ATTRIBUTE_CODES),
            base_height_ur: RecordValue::new(&parts[2], "meters", 1),
            base_height_lr: RecordValue::new(&parts[3], "meters", 1),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GF1 {
    total_coverage_code: CodeRecord,
    total_opaque_coverage_code: CodeRecord,
    total_coverage_quality_code: CodeRecord,
    low_total_coverage_code: CodeRecord,
    low_total_coverage_quality_code: CodeRecord,
    low_cloud_genus_code: CodeRecord,
    low_cloud_genus_quality_code: CodeRecord,
    low_cloud_base_height: Option<RecordValue<i32>>,
    low_cloud_base_height_quality_code: CodeRecord,
    mid_cloud_genus_code: CodeRecord,
    mid_cloud_genus_quality_code: CodeRecord,
    high_cloud_genus_code: CodeRecord,
}

impl FromStr for GF1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GF1 {
            total_coverage_code: CodeRecord::new(&parts[0], &COVERAGE_CODES),
            total_opaque_coverage_code: CodeRecord::new(&parts[1], &COVERAGE_CODES),
            total_coverage_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            low_total_coverage_code: CodeRecord::new(&parts[3], &COVERAGE_CODES),
            low_total_coverage_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            low_cloud_genus_code: CodeRecord::new(&parts[5], &LOW_CLOUD_GENUS_CODES),
            low_cloud_genus_quality_code: CodeRecord::new(&parts[6], &QUALITY_CODES),
            low_cloud_base_height: RecordValue::new(&parts[7], "meters", 1),
            low_cloud_base_height_quality_code: CodeRecord::new(&parts[8], &QUALITY_CODES),
            mid_cloud_genus_code: CodeRecord::new(&parts[9], &MID_CLOUD_GENUS_CODES),
            mid_cloud_genus_quality_code: CodeRecord::new(&parts[10], &QUALITY_CODES),
            high_cloud_genus_code: CodeRecord::new(&parts[11], &HIGH_CLOUD_GENUS_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
//GG1-6
pub struct GGX {
    coverage_code: CodeRecord,
    coverage_quality_code: CodeRecord,
    top_height: Option<RecordValue<i32>>,
    top_height_quality_code: CodeRecord,
    type_code: CodeRecord,
    type_quality_code: CodeRecord,
    top_code: CodeRecord,
    top_quality_code: CodeRecord,
}
impl FromStr for GGX {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GGX {
            coverage_code: CodeRecord::new(&parts[0], &COVERAGE_CODES),
            coverage_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            top_height: RecordValue::new(&parts[2], "meters", 1),
            top_height_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            type_code: CodeRecord::new(&parts[4], &CLOUD_TYPE_CODES),
            type_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
            top_code: CodeRecord::new(&parts[6], &TOP_CODES),
            top_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GH1 {
    avg_solar_radiation: Option<RecordValue<i32>>,
    avg_solar_radiation_quality_code: CodeRecord,
    avg_solar_radiation_quality_flag: CodeRecord,
    min_solar_radiation: Option<RecordValue<i32>>,
    min_solar_radiation_quality_code: CodeRecord,
    min_solar_radiation_quality_flag: CodeRecord,
    max_solar_radiation: Option<RecordValue<i32>>,
    max_solar_radiation_quality_code: CodeRecord,
    max_solar_radiation_quality_flag: CodeRecord,
    std_solar_radiation: Option<RecordValue<i32>>,
    std_solar_radiation_quality_code: CodeRecord,
    std_solar_radiation_quality_flag: CodeRecord,
}
impl FromStr for GH1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GH1 {
            avg_solar_radiation: RecordValue::new(&parts[0], "W/m^2", 10),
            avg_solar_radiation_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
            avg_solar_radiation_quality_flag: CodeRecord::new(&parts[2], &QUALITY_FLAG_CODES),
            min_solar_radiation: RecordValue::new(&parts[3], "W/m^2", 10),
            min_solar_radiation_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            min_solar_radiation_quality_flag: CodeRecord::new(&parts[5], &QUALITY_FLAG_CODES),
            max_solar_radiation: RecordValue::new(&parts[6], "W/m^2", 10),
            max_solar_radiation_quality_code: CodeRecord::new(&parts[7], &QUALITY_CODES),
            max_solar_radiation_quality_flag: CodeRecord::new(&parts[8], &QUALITY_FLAG_CODES),
            std_solar_radiation: RecordValue::new(&parts[9], "W/m^2", 10),
            std_solar_radiation_quality_code: CodeRecord::new(&parts[10], &QUALITY_CODES),
            std_solar_radiation_quality_flag: CodeRecord::new(&parts[11], &QUALITY_FLAG_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GJ1 {
    sunshine_duration: Option<RecordValue<i32>>,
    sunshine_duration_quality_code: CodeRecord,
}
impl FromStr for GJ1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GJ1 {
            sunshine_duration: RecordValue::new(&parts[0], "minutes", 1),
            sunshine_duration_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GK1 {
    sunshine_quantity: Option<RecordValue<i32>>,
    sunshine_quantity_quality_code: CodeRecord,
}
impl FromStr for GK1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GK1 {
            sunshine_quantity: RecordValue::new(&parts[0], "%", 1),
            sunshine_quantity_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GL1 {
    sunshine_duration: Option<RecordValue<i32>>,
    sunshine_duration_quality_code: CodeRecord,
}
impl FromStr for GL1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GL1 {
            sunshine_duration: RecordValue::new(&parts[0], "minutes", 1),
            sunshine_duration_quality_code: CodeRecord::new(&parts[1], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GM1 {
    solar_irradiance_period: Option<RecordValue<i32>>,
    global_irradiance: Option<RecordValue<i32>>,
    global_irradiance_data_flag: CodeRecord,
    global_irradiance_quality_code: CodeRecord,
    direct_beam_irradiance: Option<RecordValue<i32>>,
    direct_beam_irradiance_data_flag: CodeRecord,
    direct_beam_irradiance_quality_code: CodeRecord,
    diffuse_irradiance: Option<RecordValue<i32>>,
    diffuse_irradiance_data_flag: CodeRecord,
    diffuse_irradiance_quality_code: CodeRecord,
    uvb_global_irradiance: Option<RecordValue<i32>>,
    uvb_global_irradiance_data_flag: CodeRecord,
    uvb_global_irradiance_quality_code: CodeRecord,
}

impl FromStr for GM1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GM1 {
            solar_irradiance_period: RecordValue::new(&parts[0], "minutes", 1),
            global_irradiance: RecordValue::new(&parts[1], "W/m^2", 1),
            global_irradiance_data_flag: CodeRecord::new(&parts[2], &DATA_FLAG_CODES),
            global_irradiance_quality_code: CodeRecord::new(&parts[3], &QUALITY_CODES),
            direct_beam_irradiance: RecordValue::new(&parts[4], "W/m^2", 1),
            direct_beam_irradiance_data_flag: CodeRecord::new(&parts[5], &DATA_FLAG_CODES),
            direct_beam_irradiance_quality_code: CodeRecord::new(&parts[6], &QUALITY_CODES),
            diffuse_irradiance: RecordValue::new(&parts[7], "W/m^2", 1),
            diffuse_irradiance_data_flag: CodeRecord::new(&parts[8], &DATA_FLAG_CODES),
            diffuse_irradiance_quality_code: CodeRecord::new(&parts[9], &QUALITY_CODES),
            uvb_global_irradiance: RecordValue::new(&parts[10], "W/m^2", 1),
            uvb_global_irradiance_data_flag: CodeRecord::new(&parts[11], &DATA_FLAG_CODES),
            uvb_global_irradiance_quality_code: CodeRecord::new(&parts[12], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GN1 {
    solar_rad_period: Option<RecordValue<i32>>,
    upwell_solar_rad: Option<RecordValue<i32>>,
    upwell_solar_rad_quality_code: CodeRecord,
    downwell_thermal_if_rad: Option<RecordValue<i32>>,
    downwell_thermal_if_rad_quality_code: CodeRecord,
    upwell_thermal_if_rad: Option<RecordValue<i32>>,
    upwell_thermal_if_rad_quality_code: CodeRecord,
    photosynth_active_rad: Option<RecordValue<i32>>,
    photosynth_active_rad_quality_code: CodeRecord,
    solar_zenith_angle: Option<RecordValue<i32>>,
    solar_zenith_angle_quality_code: CodeRecord,
}
impl FromStr for GN1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GN1 {
            solar_rad_period: RecordValue::new(&parts[0], "minutes", 1),
            upwell_solar_rad: RecordValue::new(&parts[1], "W/m^2", 1),
            upwell_solar_rad_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            downwell_thermal_if_rad: RecordValue::new(&parts[3], "W/m^2", 1),
            downwell_thermal_if_rad_quality_code: CodeRecord::new(&parts[4], &QUALITY_CODES),
            upwell_thermal_if_rad: RecordValue::new(&parts[5], "W/m^2", 1),
            upwell_thermal_if_rad_quality_code: CodeRecord::new(&parts[6], &QUALITY_CODES),
            photosynth_active_rad: RecordValue::new(&parts[7], "W/m^2", 1),
            photosynth_active_rad_quality_code: CodeRecord::new(&parts[8], &QUALITY_CODES),
            solar_zenith_angle: RecordValue::new(&parts[9], "degrees", 1),
            solar_zenith_angle_quality_code: CodeRecord::new(&parts[10], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GO1 {
    net_solar_rad_period: Option<RecordValue<i32>>,
    net_solar_rad: Option<RecordValue<i32>>,
    net_solar_rad_quality_code: CodeRecord,
    net_thermal_if_rad: Option<RecordValue<i32>>,
    net_rad: Option<RecordValue<i32>>,
    net_rad_quality_code: CodeRecord,
}

impl FromStr for GO1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GO1 {
            net_solar_rad_period: RecordValue::new(&parts[0], "minutes", 1),
            net_solar_rad: RecordValue::new(&parts[1], "W/m^2", 1),
            net_solar_rad_quality_code: CodeRecord::new(&parts[2], &QUALITY_CODES),
            net_thermal_if_rad: RecordValue::new(&parts[3], "W/m^2", 1),
            net_rad: RecordValue::new(&parts[4], "W/m^2", 1),
            net_rad_quality_code: CodeRecord::new(&parts[5], &QUALITY_CODES),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GP1 {
    mod_solar_irradiance_period: Option<RecordValue<i32>>,
    mod_global_horizontal: Option<RecordValue<i32>>,
    mod_global_horizontal_source: CodeRecord,
    mog_global_horizontal_uncertainty: Option<RecordValue<i32>>,
    mod_direct_normal: Option<RecordValue<i32>>,
    mod_direct_normal_source: CodeRecord,
    mod_direct_normal_uncertainty: Option<RecordValue<i32>>,
    mod_diffuse_horizontal: Option<RecordValue<i32>>,
    mod_diffuse_horizontal_source: CodeRecord,
    mod_diffuse_horizontal_uncertainty: Option<RecordValue<i32>>,
}

impl FromStr for GP1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GP1 {
            mod_solar_irradiance_period: RecordValue::new(&parts[0], "minutes", 1),
            mod_global_horizontal: RecordValue::new(&parts[1], "W/m^2", 1),
            mod_global_horizontal_source: CodeRecord::new(
                &parts[2],
                &MOD_GLOBAL_HORIZONTAL_SOURCE_FLAGS,
            ),
            mog_global_horizontal_uncertainty: RecordValue::new(&parts[3], "%", 1),
            mod_direct_normal: RecordValue::new(&parts[4], "W/m^2", 1),
            mod_direct_normal_source: CodeRecord::new(
                &parts[5],
                &MOD_GLOBAL_HORIZONTAL_SOURCE_FLAGS,
            ),
            mod_direct_normal_uncertainty: RecordValue::new(&parts[6], "%", 1),
            mod_diffuse_horizontal: RecordValue::new(&parts[7], "W/m^2", 1),
            mod_diffuse_horizontal_source: CodeRecord::new(
                &parts[8],
                &MOD_GLOBAL_HORIZONTAL_SOURCE_FLAGS,
            ),
            mod_diffuse_horizontal_uncertainty: RecordValue::new(&parts[9], "%", 1),
        })
    }
}

#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GQ1 {
    solar_angle_time: Option<RecordValue<i32>>,
    mean_zenith_angle: Option<RecordValue<i32>>,
    mean_zenith_angle_quality: CodeRecord,
    mean_azimuth_angle: Option<RecordValue<i32>>,
    mean_azimuth_angle_quality: CodeRecord,
}

impl FromStr for GQ1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GQ1 {
            solar_angle_time: RecordValue::new(&parts[0], "minutes", 1),
            mean_zenith_angle: RecordValue::new(&parts[1], "degrees", 10),
            mean_zenith_angle_quality: CodeRecord::new(&parts[2], &QUALITY_CODES),
            mean_azimuth_angle: RecordValue::new(&parts[3], "degrees", 10),
            mean_azimuth_angle_quality: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}
#[derive(DeserializeFromStr, Serialize, Debug, PartialEq)]
pub struct GR1 {
    et_rad_time: Option<RecordValue<i32>>,
    et_rad_horizontal_surface: Option<RecordValue<i32>>,
    et_rad_horizontal_surface_quality: CodeRecord,
    et_rad_direct_normal: Option<RecordValue<i32>>,
    et_rad_direct_normal_quality: CodeRecord,
}

impl FromStr for GR1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_parts(s);
        Ok(GR1 {
            et_rad_time: RecordValue::new(&parts[0], "minutes", 1),
            et_rad_horizontal_surface: RecordValue::new(&parts[1], "W/m^2", 1),
            et_rad_horizontal_surface_quality: CodeRecord::new(&parts[2], &QUALITY_CODES),
            et_rad_direct_normal: RecordValue::new(&parts[3], "W/m^2", 1),
            et_rad_direct_normal_quality: CodeRecord::new(&parts[4], &QUALITY_CODES),
        })
    }
}
