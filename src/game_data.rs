use crate::fe::InfoDisplays;
use crate::fileload::{bool_from_string, f32_from_string, i32_from_string, Hashable};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Error};
use std::path::PathBuf;

trait CsvLoad {
    fn load_csv<'de, T>(path: PathBuf) -> Result<Vec<T>, Error>
    where
        T: Deserialize<'de>,
        T: Hashable,
    {
        let mut rdr = match csv::ReaderBuilder::new().delimiter(b';').from_path(path) {
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
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Building {
    #[serde(deserialize_with = "bool_from_string")]
    overclockable: Option<bool>,
    name: String,
    #[serde(deserialize_with = "f32_from_string", rename = "powerUsage")]
    power_usage: Option<f32>,
    #[serde(deserialize_with = "f32_from_string", rename = "powerGenerated")]
    power_generated: Option<f32>,
    #[serde(deserialize_with = "f32_from_string")]
    inputs: Option<f32>,
    #[serde(deserialize_with = "f32_from_string")]
    outputs: Option<f32>,
    #[serde(deserialize_with = "f32_from_string")]
    width: Option<f32>,
    #[serde(deserialize_with = "f32_from_string")]
    length: Option<f32>,
    #[serde(deserialize_with = "f32_from_string")]
    height: Option<f32>,
}

impl CsvLoad for Building {}
impl Hashable for Building {
    fn get_key(&self) -> String {
        String::clone(&self.name)
    }
}
impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}
impl InfoDisplays for Building {
    fn nfmt(&self) -> Result<String, Error> {
        Ok(format!("{}", self.name.clone()))
    }
    fn dfmt(&self) -> Result<String, Error> {
        Ok(format!(
            "{:?}  {:?}  {:?}  {:?}",
            self.name, self.inputs, self.outputs, self.power_usage
        ))
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RecipeCSV {
    #[serde(rename = "recipeName")]
    recipe_name: String,
    #[serde(rename = "alternateRecipe", deserialize_with = "bool_from_string")]
    alternate_recipe: Option<bool>,
    #[serde(rename = "mainRecipe", deserialize_with = "bool_from_string")]
    main_recipe: Option<bool>,
    #[serde(rename = "avgPower", deserialize_with = "i32_from_string")]
    avg_power: Option<i32>,
    #[serde(rename = "producedIn")]
    produced_in: Option<String>,
    #[serde(rename = "product")]
    product: Option<String>,
    #[serde(rename = "product2")]
    product2: Option<String>,
    #[serde(rename = "product3")]
    product3: Option<String>,
    #[serde(rename = "product4")]
    product4: Option<String>,
    #[serde(rename = "productsPerMinute", deserialize_with = "i32_from_string")]
    products_per_minute: Option<i32>,
    #[serde(rename = "productsPerMinute2", deserialize_with = "i32_from_string")]
    products_per_minute2: Option<i32>,
    #[serde(rename = "productsPerMinute3", deserialize_with = "i32_from_string")]
    products_per_minute3: Option<i32>,
    #[serde(rename = "productsPerMinute4", deserialize_with = "i32_from_string")]
    products_per_minute4: Option<i32>,
    #[serde(rename = "ingredient1")]
    ingredient1: Option<String>,
    #[serde(rename = "ingredient2")]
    ingredient2: Option<String>,
    #[serde(rename = "ingredient3")]
    ingredient3: Option<String>,
    #[serde(rename = "ingredient4")]
    ingredient4: Option<String>,
    #[serde(rename = "quantity1", deserialize_with = "i32_from_string")]
    quantity1: Option<i32>,
    #[serde(rename = "quantity2", deserialize_with = "i32_from_string")]
    quantity2: Option<i32>,
    #[serde(rename = "quantity3", deserialize_with = "i32_from_string")]
    quantity3: Option<i32>,
    #[serde(rename = "quantity4", deserialize_with = "i32_from_string")]
    quantity4: Option<i32>,
}

#[derive(Debug)]
pub struct Recipe {
    recipe_name: String,
    alternate_recipe: bool,
    main_recipe: bool,
    avg_power: i32,
    produced_in: String,
    products: Vec<(String, i32)>,
    ingredients: Vec<(String, i32)>,
}
impl Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.recipe_name)
    }
}
impl Hashable for RecipeCSV {
    fn get_key(&self) -> String {
        return self.recipe_name.clone();
    }
}
