mod temperature;
mod length;
mod history;

use std::str::FromStr;
use clap::{Parser, Subcommand};
use crate::{ 
    history::{read_history, write_history, ConvertionRecord}, length::{cm_to_inch,cm_to_km, cm_to_miles, inch_to_cm, inch_to_km, inch_to_miles, km_to_cm, km_to_inch, km_to_miles, miles_to_cm, miles_to_inch, miles_to_km, LengthUnit}, temperature::{celcius_to_fahrenheit, celcius_to_kelvin, fahrenheit_to_celcius, fahrenheit_to_kelvin, kelvin_to_celcius, kelvin_to_fahrenheit, TemperatureUnit}
};

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        #[arg(long)]
        value: f64,
    },
    List,
    History,
}

#[derive(Parser)]
#[command(name = "unitconv", version = "1.0", about = "Aplikasi konversi suhu dan panjang")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug)]
enum Unit {
    Temperature(TemperatureUnit),
    Length(LengthUnit),
}

struct UnitList {
    category: String,
    name: Unit,
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(temp) = TemperatureUnit::from_str(s) {
            return Ok(Unit::Temperature(temp));
        }

        if let Ok(len) = LengthUnit::from_str(s) {
            return Ok(Unit::Length(len));
        }

        Err(format!("[ERROR] Satuan tujuan {} tidak dikenali.", s))
    }
}

impl Unit {
    fn as_str(&self) -> &'static str {
        match self {
            Unit::Length(len) => len.as_str(),
            Unit::Temperature(temp) => temp.as_str().0,
        }
    }

    fn format_str(&self) -> &'static str {
        match self {
            Unit::Length(len) => len.as_str(),
            Unit::Temperature(temp) => temp.as_str().1,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let supported_unit_lists: Vec<UnitList> = vec![
        UnitList{ category: String::from("suhu"), name: Unit::Temperature(TemperatureUnit::Celsius) },
        UnitList{ category: String::from("suhu"), name: Unit::Temperature(TemperatureUnit::Fahrenheit) },
        UnitList{ category: String::from("suhu"), name: Unit::Temperature(TemperatureUnit::Kelvin) },
        UnitList{ category: String::from("panjang"), name: Unit::Length(LengthUnit::Cm) },
        UnitList{ category: String::from("panjang"), name: Unit::Length(LengthUnit::Inch) },
        UnitList{ category: String::from("panjang"), name: Unit::Length(LengthUnit::Km) },
        UnitList{ category: String::from("panjang"), name: Unit::Length(LengthUnit::Miles) },
    ];
    let mut historys = read_history().unwrap();

    match cli.command {
        Commands::Convert { from, to, value } => {
            let from_unit = match Unit::from_str(&from) {
                Ok(u) => u,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let to_unit = match Unit::from_str(&to) {
                Ok(u) => u,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            match (from_unit.clone(), to_unit.clone()) {
                (Unit::Temperature(TemperatureUnit::Celsius), Unit::Temperature(TemperatureUnit::Fahrenheit)) 
                => convert(celcius_to_fahrenheit, value, &from_unit, &to_unit, &mut historys),

                (Unit::Temperature(TemperatureUnit::Celsius), Unit::Temperature(TemperatureUnit::Kelvin)) 
                => convert(celcius_to_kelvin, value, &from_unit, &to_unit, &mut historys),

                (Unit::Temperature(TemperatureUnit::Fahrenheit), Unit::Temperature(TemperatureUnit::Celsius)) 
                => convert(fahrenheit_to_celcius, value, &from_unit, &to_unit, &mut historys),

                (Unit::Temperature(TemperatureUnit::Fahrenheit), Unit::Temperature(TemperatureUnit::Kelvin)) 
                => convert(fahrenheit_to_kelvin, value, &from_unit, &to_unit, &mut historys),

                (Unit::Temperature(TemperatureUnit::Kelvin), Unit::Temperature(TemperatureUnit::Celsius)) 
                => convert(kelvin_to_celcius, value, &from_unit, &to_unit, &mut historys),

                (Unit::Temperature(TemperatureUnit::Kelvin), Unit::Temperature(TemperatureUnit::Fahrenheit)) 
                => convert(kelvin_to_fahrenheit, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Cm), Unit::Length(LengthUnit::Inch))
                => convert(cm_to_inch, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Cm), Unit::Length(LengthUnit::Km))
                => convert(cm_to_km, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Cm), Unit::Length(LengthUnit::Miles))
                => convert(cm_to_miles, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Inch), Unit::Length(LengthUnit::Cm))
                => convert(inch_to_cm, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Inch), Unit::Length(LengthUnit::Km))
                => convert(inch_to_km, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Inch), Unit::Length(LengthUnit::Miles))
                => convert(inch_to_miles, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Km), Unit::Length(LengthUnit::Cm))
                => convert(km_to_cm, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Km), Unit::Length(LengthUnit::Inch))
                => convert(km_to_inch, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Km), Unit::Length(LengthUnit::Miles))
                => convert(km_to_miles, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Miles), Unit::Length(LengthUnit::Cm))
                => convert(miles_to_cm, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Miles), Unit::Length(LengthUnit::Inch))
                => convert(miles_to_inch, value, &from_unit, &to_unit, &mut historys),

                (Unit::Length(LengthUnit::Miles), Unit::Length(LengthUnit::Km))
                => convert(miles_to_km, value, &from_unit, &to_unit, &mut historys),

                _ => println!("Error: [ERROR] Tidak dapat mengkonversi satuan yang berbeda kategori: {:?} -> {:?}", from, to),
            }
        },
        Commands::List => {
            println!("Satuan yang didukung: ");
            for (index, unit) in supported_unit_lists.iter().enumerate() {
                println!("{}. [{}] {}", index + 1, unit.category, unit.name.as_str());
            }
        },
        Commands::History => {
            println!("Riwayat Konversi: ");

            if historys.len() == 0 {
                println!("Belum ada histori yang tercatat");
            } else {
                for (index, history) in historys.iter().enumerate() {
                    println!("{}. {} = {}", index + 1, history.input, history.output);
                }
            }
        }
    }
}

fn convert<F>(fnc: F, input: f64, from: &Unit, to: &Unit, historys: &mut Vec<ConvertionRecord>) where F: Fn(f64) -> f64 {
    let result = fnc(input);
    let record = ConvertionRecord {
        input: format!("{} {}", input, from.format_str()),
        output: format!("{} {}", result, to.format_str()),
    };
    historys.push(record);

    if let Err(_e) = write_history(&historys) {
        eprint!("Gagal menyimpan history");
        return;
    }

    println!("{} {} = {} {}", input, from.format_str(), result, to.format_str());
}