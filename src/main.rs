mod fe;
mod fileload;
mod pipeoptz;
mod game_data;

use game_data::{Building, RecipeCSV};
use fileload::csv_to_hash;
use inquire::Select;
use std::{collections::HashMap, env::current_dir};

fn main() -> std::io::Result<()> {

    let recipe_options = recipes.keys().clone().collect();
    let chosen_recipe = Select::new("What would you like to make?", recipe_options)
        .prompt()
        .unwrap();


    Ok(())
}
