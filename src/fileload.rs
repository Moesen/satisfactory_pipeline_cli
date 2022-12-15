use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use std::{collections::HashMap, error::Error, path::PathBuf};
use crate::game_data::{Building, Recipe};

pub trait Hashable {
    fn get_key(&self) -> String;
}

pub fn bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer) {
        Ok(1) => Ok(Some(true)),
        _ => Ok(Some(false))
    }
}

pub fn f32_from_string<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    match f32::deserialize(deserializer) {
        Ok(val) => Ok(Some(val)),
        Err(_) => Ok(None),
    }
}

pub fn i32_from_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    match i32::deserialize(deserializer) {
        Ok(val) => Ok(Some(val)),
        Err(_) => Ok(None)
    }
}

pub fn csv_to_hash<T>(csv_path: PathBuf) -> Result<HashMap<String, T>, Box<dyn Error>>
where
    T: DeserializeOwned,
    T: Hashable,
{
    let mut rdr = match csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)
    {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    let mut map: HashMap<String, T> = HashMap::new();
    for result in rdr.deserialize() {
        let res: T = result?;
        let key = res.get_key();
        map.insert(key, res);
    }
    Ok(map)
}

pub fn load_game_files() -> Result<(HashMap<String, Recipe>, HashMap<String, Building>), > {
    let cur_dir = current_dir()?;
    let dir_path = cur_dir.parent().expect("Could not find parent directory");
    let data_path = dir_path.join("data");

    let buildings_path = data_path.join("buildings.csv");
    let buildings: HashMap<String, Building> = csv_to_hash(buildings_path).unwrap();

    let recipes_path = data_path.join("recipes.csv");
    let recipes: HashMap<String, RecipeCSV> = csv_to_hash(recipes_path).unwrap();
}
