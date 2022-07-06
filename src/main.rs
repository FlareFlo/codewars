#![allow(dead_code)]

use std::{env, fs};

mod solutions;


// Creates a new module with the name provided via argument
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut name = args.get(1).unwrap_or_else(|| {
            panic!("No folder name provided");
        }).clone();
    name = name.replace(" ", "_").to_lowercase();

    // Make new folder
    fs::create_dir("src/solutions/".to_owned() + &name).unwrap();

    // modify mod rs
    let core_mod = fs::read_to_string("src/solutions/mod.rs").unwrap();
    let new_core_mod = format!("{core_mod}\nmod {name};");
    fs::write("src/solutions/mod.rs", new_core_mod).unwrap();

    // mod folder
    fs::write(format!("src/solutions/{name}/mod.rs"), format!("mod solution;")).unwrap();

    // create solution
    fs::write(format!("src/solutions/{name}/solution.rs"), "").unwrap();
}