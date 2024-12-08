// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unsafe_code)]
#[cfg(target_arch = "wasm32")]


// mod monsters;
// mod order;
// mod mapped;

use wasm_bindgen::prelude::*; 
use rand::prelude::*;
use std::io;


slint::include_modules!();

use std::rc::Rc;

use slint::{Model, ModelExt, ModelRc, SharedString, StandardListViewItem, VecModel};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = TableViewPage::new().unwrap();

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    // generate rows and columns
    for r in 1..2 { // rows
        let items: Rc<VecModel<StandardListViewItem>> = Rc::new(VecModel::default());

        for c in 1..5 { // columns
            if c == 1 {
                //let initiative = rand::random::<u8>() % 100;
                let input_num = input_request(0);
                items.push(slint::format!("{input_num}").into());
            } else if c == 2 {
                println!("Please enter your Name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                println!("You enterd an name {}", name.trim());
                items.push(slint::format!("{name}").into());
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