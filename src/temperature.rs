use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for TemperatureUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" => Ok(Self::Celsius),
            "fahrenheit" => Ok(Self::Fahrenheit),
            "kelvin" => Ok(Self::Kelvin),
            _ => Err(s.to_string()),
        }
    }
}

impl TemperatureUnit {
    pub fn as_str(&self) -> (&'static str, &'static str) {
        match self {
            TemperatureUnit::Celsius => ("celsius", "°C"),
            TemperatureUnit::Fahrenheit => ("fahrenheit", "°F"),
            TemperatureUnit::Kelvin => ("kelvin", "°K")
        }
    }
}

const KELVIN_OFFSET: f64 = 273.15;
const NINE_FIFTH: f64 = 9.0 / 5.0;
const FIFTH_NINE: f64 = 5.0 / 9.0;

pub fn celcius_to_fahrenheit(celcius_temp: f64) -> f64 {
    NINE_FIFTH * celcius_temp + 32.0
}

pub fn celcius_to_kelvin(celcius_temp: f64) -> f64 {
    celcius_temp + KELVIN_OFFSET
}

pub fn fahrenheit_to_celcius(fahrenheit_temp: f64) -> f64 {
    FIFTH_NINE * (fahrenheit_temp - 32.0)
}

pub fn fahrenheit_to_kelvin(fahrenheit_temp: f64) -> f64 {
    FIFTH_NINE * (fahrenheit_temp - 32.0) + KELVIN_OFFSET
}

pub fn kelvin_to_celcius(kelvin_temp: f64) -> f64 {
    kelvin_temp - KELVIN_OFFSET
}

pub fn kelvin_to_fahrenheit(kelvin_temp: f64) -> f64 {
    NINE_FIFTH * (kelvin_temp - KELVIN_OFFSET) + 32.0
}