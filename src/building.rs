use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fmt::Display;

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        1 => Ok(true),
        _ => Ok(false),
    }
}

fn num_from_string<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    match f32::deserialize(deserializer) {
        Ok(val) => Ok(val),
        Err(_) => Ok(0.0),
    }
}

#[derive(Debug, Deserialize)]
pub struct Building {
    #[serde(deserialize_with = "bool_from_int")]
    overclockable: bool,
    name: String,
    #[serde(deserialize_with = "num_from_string", rename="powerUsage")]
    power_usage: f32,
    #[serde(deserialize_with = "num_from_string", rename="powerGenerated")]
    power_generated: f32,
    #[serde(deserialize_with = "num_from_string")]
    inputs: f32,
    #[serde(deserialize_with = "num_from_string")]
    outputs: f32,
    #[serde(deserialize_with = "num_from_string")]
    width: f32,
    #[serde(deserialize_with = "num_from_string")]
    length: f32,
    #[serde(deserialize_with = "num_from_string")]
    height: f32,
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn load_buildings() -> Result<Vec<Building>, Box<dyn Error>> {
    let build_file_path = "../data/buildings.csv";
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(build_file_path)?;
    let mut buildings: Vec<Building> = Vec::new();
    for result in rdr.deserialize() {
        let b: Building = result?;
        buildings.push(b);
    }
    Ok(buildings)
}
