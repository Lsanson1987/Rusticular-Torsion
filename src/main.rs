// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unsafe_code)]
use std::alloc::System;
#[cfg(target_arch = "wasm32")]

use rand::Rng;
use std::{collections::HashMap, hash::Hash, iter};
//use wasm_bindgen::prelude::*; 
use rand::prelude::*;
use std::io::{self, BufReader};


slint::include_modules!();

use std::rc::Rc;
//use csv::Reader;
// use std::error::Error;
use std::fs::File;
use std::path::Path;


use slint::{Model, ModelExt, ModelRc, SharedString, StandardListViewItem, VecModel};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = Rc::new(TableViewPage::new().unwrap());

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    app.global::<TableViewPageAdapter>().set_row_data(row_data.clone().into());

    app.on_add_row({
        // have to clone data and wrap in RC for shared ownership because dumb rust ownership memory management
        let app_clone = app.clone(); 
        let row_data_clone = row_data.clone(); 
        let monster_data = monster_data_base();

        // converts variables captured by reference to variables captured by value.
        move || {
            let items: Rc<VecModel<StandardListViewItem>> = Rc::new(VecModel::default());
    
            // Retrieve the name entered by the user
            let current_name = app_clone.get_current_name().trim().to_string();

            let mut found: bool = false;
            // check for autofill in the monster database
            for (key, value) in &monster_data {
                if *key.trim().to_lowercase() == current_name.trim().to_lowercase() {
                    found = true;
                    let initive_rand: i32 = roll_dice(1, 20, ((value.get_init() - 10) / 2).try_into().unwrap());
                    items.push(slint::format!("{:?}", initive_rand).into());
                    items.push(slint::format!("{current_name}").into());
                    items.push(slint::format!("{:?}", value.display_armor_class()).into());
                    items.push(slint::format!("{:?}", value.display_hit_points()).into());
                    break;
                } 
            }

            // If no match, use manual inputs
            if !found {
                let current_init: SharedString = app_clone.get_current_initiative();
                let current_hp = app_clone.get_current_hp();
                let current_ac = app_clone.get_current_ac();

                items.push(slint::format!("{current_init}").into());
                items.push(slint::format!("{current_name}").into());
                items.push(slint::format!("{current_ac}").into());
                items.push(slint::format!("{current_hp}").into());
            }

            // Add the row to row_data
            row_data_clone.push(items.into());
        }
    });

    app.on_delete_row_by_name({
        // Clone the data model for shared ownership
        let app_clone = app.clone(); 
        let row_data_clone = row_data.clone(); 
    
        move || {
            // Get the name & initative to delete
            let current_name = app_clone.get_current_name().trim().to_string(); 
            let current_initiative = app_clone.get_current_initiative().trim().to_string(); 
    
            // Find the index of the row with the specified name and initiative
            if let Some(index) = row_data_clone.iter().position(|row| {
                let name = row.row_data(1).unwrap().text.to_string();
                let initiative = row.row_data(0).unwrap().text.to_string();
                // Both name and initiative must match
                name.trim().eq_ignore_ascii_case(&current_name)
                    && initiative.trim() == current_initiative 
            }) {
                // Remove the row
                row_data_clone.remove(index); 
            } else {
                eprintln!(
                    "No row found with Name: '{}' and Initiative: '{}'",
                    current_name, current_initiative
                );
            }
        }
    });
    
    // callback to update the row data
    app.global::<TableViewPageAdapter>().set_row_data(row_data.clone().into());
    // callback to update the row data after filtering and sorting
    app.global::<TableViewPageAdapter>().on_filter_sort_model(filter_sort_model);
    // run the app and show the window
    app.run().unwrap();
}


// function callback to filter and sort the model
fn filter_sort_model(
    source_model: ModelRc<ModelRc<StandardListViewItem>>,
    filter: SharedString,
    sort_index: i32,
    sort_ascending: bool,
) -> ModelRc<ModelRc<StandardListViewItem>> {
    // have to do this because of Rust's ownership rules
    let mut model = source_model.clone();


    if !filter.is_empty() {
        // note: this code is taken from one of the examples in the documentation, https://docs.slint.dev/latest/demos/gallery/
        let filter = filter.to_lowercase();

        // filter by first row
        model =
            Rc::new(source_model.clone().filter(move |e| {
                e.row_data(1).unwrap().text.to_lowercase().contains(filter.as_str())
            }))
            .into();
    }

    if sort_index >= 0 {
        // this code has been adjusted to account for the fact that the data in the column is numeric
        model = Rc::new(
            model.clone().sort_by(move |r_a, r_b| {
                let c_a = r_a.row_data(sort_index as usize).unwrap();
                let c_b = r_b.row_data(sort_index as usize).unwrap();

                // Use text-to-integer conversion if the data in the column is numeric
                let a = c_a.text.parse::<i32>().unwrap_or(0);
                let b = c_b.text.parse::<i32>().unwrap_or(0);

                if sort_ascending {
                    a.cmp(&b) // Ascending order
                } else {
                    b.cmp(&a) // Descending order
                }
            }),
        )
        .into();
    }

    model
}
//the data base that is holding all the monsters
//created using previous python file
fn monster_data_base() -> HashMap<String, Monster> {
    let mut monster_list: HashMap<String, Monster> = HashMap::new();
    let file_path = "monsters.csv";
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return monster_list; // return an empty map in case of error
        }
    };
    
    let reader = io::BufReader::new(file);
    let mut first_line = true;

    for line in io::BufRead::lines(reader) {
        match line {
            Ok(line) => {
                if first_line {
                    first_line = false;
                    continue;
                }
                let values: Vec<String> = line.split(',')
                                              .map(|s| s.to_string())
                                              .collect();

                let ac: i64 = values[1].parse().unwrap_or(0);
                let hp: i64 = values[2].parse().unwrap_or(0);
                let dwa: i64 = values[3].parse().unwrap_or(0);
                let monster = Monster::new(&values[0], ac, hp, dwa, "");
                monster_list.insert(values[0].clone(), monster);
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    monster_list
}


//moster struct used in the hasmap
pub struct Monster {
    name: String,
    initiative: i64,
    hit_points: i64,
    armor_class: i64,
    notes: String,
}
//getters and setter for the monster struct and other various functions
impl Monster {
    pub fn new(name: &str, initiative: i64, hit_points: i64, armor_class: i64, notes: &str) -> Monster {
      Monster {  name: name.to_string(),
        initiative,
        hit_points,
        armor_class,
        notes: notes.to_string(),
      }
    }   
    pub fn copy(&self) -> Monster {
        Monster {
            name: self.name.clone(),
            initiative: self.initiative,
            hit_points: self.hit_points,
            armor_class: self.armor_class,
            notes: self.notes.clone(),
        }
    }
    pub fn display_name(&self) {
        println!("{}", self.name);
    }
    pub fn display_initiative(&self) {
        println!("{}", self.initiative);
    }
    pub fn get_init(&self) -> i64 {
        self.initiative
    }
    pub fn display_hit_points(&self) -> i64 {
        self.hit_points
    }
    pub fn display_armor_class(&self) -> i64 {
        self.armor_class
    }
    pub fn display_notes(&self) {
        println!("{}", self.notes);
    }
    pub fn change_initiative(&mut self, int: i64) {
        self.initiative = int;
    }
    pub fn change_armor(&mut self, int: i64) {
        self.armor_class = int;
    }
    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn change_note(&mut self, name: &str) {
        self.notes = name.to_string();
    }
    pub fn damage(&mut self, x: i64) {
        self.hit_points = self.hit_points - x;
    }
    pub fn heal(&mut self, x: i64) {
        self.hit_points = self.hit_points + x;
    }
}

//allows for dice rolls
//select the number of dice, number of sides on that dicce, and if there are any modifiers to add at end of rolling

// needs to be int cuz modifiers can be negative
pub fn roll_dice(num_dice: i32, sides: i32, modifier: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut total = 0;
    for _ in 0..num_dice {
        let roll = rng.gen_range(1..=sides);
        print!("You got: {}", roll);
        print!("You got with modfier: {}", roll + modifier);
        total += roll;
    }
    print!("In total you got raw: {}", total);
    print!("In total you got with modifier: {}", modifier + total);
    total + modifier
}
