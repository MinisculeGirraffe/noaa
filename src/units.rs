use phf::phf_map;

pub static _UNITS: phf::Map<&'static str, &'static str> = phf_map! {
    "Angular Degrees" => "°",
    "Kilometers" => "km",
    "cubic meters per second" => "m^3/s",
    "Geopotential Meters" => "m",
    "Meters" => "m",
    "millimeters" => "mm",
    "centimeters" => "cm",
    "Inches" => "in",
    "meters per second" => "m/s",
    "Statute Miles" => "mi",
    "Degrees Celsius" => "°C",
    "Hectopascals" => "hPa",
    "Hours" => "h",
    "Minutes" => "min",
    "Seconds" => "s",
    "Days" => "d",
    "rotations per second" => "r/s",
    "percent" => "%",
    "Hertz" => "Hz",
    "Watts" => "W",
    "watts per square meter" => "W/m^2",
    "milli-watts per square meter" => "mW/m²",
    "volts" => "V",
    "ohms" => "Ω",
    "" => ""
};