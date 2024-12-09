// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unsafe_code)]
use std::alloc::System;
#[cfg(target_arch = "wasm32")]


// mod order;
// mod mapped;
//mod monsters;
//use monsters::Monster;
use std::{collections::HashMap, hash::Hash, iter};
//use wasm_bindgen::prelude::*; 
use rand::prelude::*;
use std::collections::{hash_map, HashMap};
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
    //file data base read 
    // let temp_map: HashMap<String, Monster>  = monster_data_base();
    // let mut monster_list: HashMap<&str, Monster> = convert_hashmap(temp_map);
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = TableViewPage::new().unwrap();

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    // generate rows and columns
    for r in 1..2 { // rows
        let items: Rc<VecModel<StandardListViewItem>> = Rc::new(VecModel::default());
        let temp_map: HashMap<String, Monster>  = monster_data_base();
        //let mut monster_list: HashMap<&str, Monster> = convert_hashmap(temp_map);
        for c in 1..5 { // columns
            if c == 2 {
                //let initiative = rand::random::<u8>() % 100;
                let input_num = input_request(0);
                items.push(slint::format!("{input_num}").into());
            } else if c == 1 {
                println!("Please enter your Name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                println!("You enterd an name {}", name.trim());
                println!("You enterd an name {}", &name);
                items.push(slint::format!("{name}").into());
                if temp_map.contains_key(&name) {
                        println!("AAAAAAAAAAAAAa");
                        //fix this issue by havving pro[er extraction]
                        items.push(slint::format!("{:?}", temp_map.get_v(&name).display_initiative()).into());
                        items.push(slint::format!("{:?}", temp_map.get_key_value(&name).display_armor_class()).into());
                        items.push(slint::format!("{:?}", temp_map.get_key_value(&name).display_hit_points()).into());
                        break;
                }
            } else if c == 3 {
                //let ac = 10 + r * 2;
                let input_num = input_request(1);
                items.push(slint::format!("{input_num}").into());
            } else if c == 4 {
                //let hp = 100 + r * 10;
                let input_num = input_request(2);
                items.push(slint::format!("{input_num}").into());
            }
        }
        row_data.push(items.into());
    }


    app.global::<TableViewPageAdapter>().set_row_data(row_data.clone().into());
    app.global::<TableViewPageAdapter>().on_filter_sort_model(filter_sort_model);

    app.run().unwrap();
}
fn input_request (version: u8 ) -> i32 {
    match version {
        0 => println!("Please enter an Initiative:"),
        1 => println!("Please enter an AC:"),
        2 => println!("Please enter an HP:"),
        _ => println!("As long the Earth, Sun, and Moon exist, everything will be alright."),
    }
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read line");
    let input_num: i32 = input_str.trim().parse().expect("Please enter a valid integer");
    println!("You entered: {}", input_num);
    return input_num;
}
fn filter_sort_model(
    source_model: ModelRc<ModelRc<StandardListViewItem>>,
    filter: SharedString,
    sort_index: i32,
    sort_ascending: bool,
) -> ModelRc<ModelRc<StandardListViewItem>> {
    let mut model = source_model.clone();

    if !filter.is_empty() {
        let filter = filter.to_lowercase();

        // filter by first row
        model =
            Rc::new(source_model.clone().filter(move |e| {
                e.row_data(0).unwrap().text.to_lowercase().contains(filter.as_str())
            }))
            .into();
    }

    if sort_index >= 0 {
        model = Rc::new(model.clone().sort_by(move |r_a, r_b| {
            let c_a = r_a.row_data(sort_index as usize).unwrap();
            let c_b = r_b.row_data(sort_index as usize).unwrap();

            if sort_ascending {
                c_a.text.cmp(&c_b.text)
            } else {
                c_b.text.cmp(&c_a.text)
            }
        }))
        .into();
    }

    model
}

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

                let ac: u64 = values[1].parse().unwrap_or(0);
                let hp: u64 = values[2].parse().unwrap_or(0);
                let dwa: u64 = values[3].parse().unwrap_or(0);
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

fn check_if_name(monster_list: &HashMap<String, Monster>, name: String) -> bool {
    return monster_list.contains_key(&name);
}


pub struct Monster {
    name: String,
    initiative: u64,
    hit_points: u64,
    armor_class: u64,
    notes: String,
}

impl Monster {
    pub fn new(name: &str, initiative: u64, hit_points: u64, armor_class: u64, notes: &str) -> Monster {
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
    pub fn display_hit_points(&self) {
        println!("{}", self.hit_points);
    }
    pub fn display_armor_class(&self) {
        println!("{}", self.armor_class);
    }
    pub fn display_notes(&self) {
        println!("{}", self.notes);
    }
    pub fn change_initiative(&mut self, int: u64) {
        self.initiative = int;
    }
    pub fn change_armor(&mut self, int: u64) {
        self.armor_class = int;
    }
    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn change_note(&mut self, name: &str) {
        self.notes = name.to_string();
    }
    pub fn damage(&mut self, x: u64) {
        self.hit_points = self.hit_points - x;
    }
    pub fn heal(&mut self, x: u64) {
        self.hit_points = self.hit_points + x;
    }
}