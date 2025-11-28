use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertionRecord {
    pub input: String,
    pub output: String, 
}

const PATH: &str = "history.json";

pub fn write_history(records: &Vec<ConvertionRecord>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&records)?;
    std::fs::write(PATH, json)?;
    Ok(())
}

pub fn read_history() -> Result<Vec<ConvertionRecord>, Box<dyn std::error::Error>> {
    if std::path::Path::new(PATH).exists() {
        let data = std::fs::read_to_string(PATH)?;

        if data.trim().is_empty() {
            return Ok(vec![]);
        }

        let records = serde_json::from_str(&data)?;
        Ok(records)
    } else {
        Ok(vec![])
    }
}