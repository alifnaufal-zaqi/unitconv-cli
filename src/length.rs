use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum LengthUnit {
    Cm,
    Inch,
    Km,
    Miles,
}

impl FromStr for LengthUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cm" => Ok(Self::Cm),
            "inch" => Ok(Self::Inch),
            "km" => Ok(Self::Km),
            "miles" => Ok(Self::Miles),
            _ => Err(format!("Error: [ERROR] Satuan {} tidak dikenali", s)),
        }
    }
}

impl LengthUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            LengthUnit::Cm => "cm",
            LengthUnit::Inch => "inch",
            LengthUnit::Km => "km",
            LengthUnit::Miles => "miles",
        }
    }
}

const CM_PER_KM: f64 = 100_000.0;
const CM_PER_INCH: f64 = 2.54;
const CM_PER_MILES: f64 = 160_934.4;
const INCH_PER_MILES: f64 = 63_360.0;
const KM_PER_MILES: f64 = 1.609_344;
const KM_PER_INCH: f64 = 39_370.07874;

pub fn cm_to_km(cm_length: f64) -> f64 {
    cm_length / CM_PER_KM
}

pub fn cm_to_inch(cm_length: f64) -> f64 {
    cm_length / CM_PER_INCH
}

pub fn cm_to_miles(cm_length: f64) -> f64 {
    cm_length / CM_PER_MILES
}

pub fn inch_to_cm(inch_length: f64) -> f64 {
    inch_length * CM_PER_INCH
}

pub fn inch_to_km(inch_length: f64) -> f64 {
    inch_length / KM_PER_INCH
}

pub fn inch_to_miles(inch_length: f64) -> f64 {
    inch_length / INCH_PER_MILES
}

pub fn km_to_cm(km_length: f64) -> f64 {
    km_length * CM_PER_KM
}

pub fn km_to_inch(km_length: f64) -> f64 {
    km_length * KM_PER_INCH
}

pub fn km_to_miles(km_length: f64) -> f64 {
    km_length / KM_PER_MILES
}

pub fn miles_to_cm(miles_length: f64) -> f64 {
    miles_length * CM_PER_MILES
}

pub fn miles_to_inch(miles_length: f64) -> f64 {
    miles_length * INCH_PER_MILES
}

pub fn miles_to_km(miles_length: f64) -> f64 {
    miles_length * KM_PER_MILES
}