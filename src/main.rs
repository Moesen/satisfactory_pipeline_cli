use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use std::process;
mod building;

fn main() {
    let buildings = match building::load_buildings() {
        Ok(val) => val,
        Err(err) => {
            println!(
                "Error loading buildings\n
                msg: {}",
                err
            );
            process::exit(-1);
        }
    };
    let formatter: MultiOptionFormatter<&str> = &|a| format!("{} different buildings", a.len());
    let ans = MultiSelect::new("Select the building: ", buildings).prompt();        
}
