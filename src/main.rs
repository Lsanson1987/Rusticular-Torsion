// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unsafe_code)]
#[cfg(target_arch = "wasm32")]

// mod monsters;
// mod order;
// mod mapped;

use wasm_bindgen::prelude::*;

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
    for r in 1..5 { // rows
        let items: Rc<VecModel<StandardListViewItem>> = Rc::new(VecModel::default());

        for c in 1..5 { // columns
            if c == 1 {
                items.push(slint::format!("{r}").into());
            } else if c == 2 {
                items.push(slint::format!("Monster").into());
            } else if c == 3 {
                let ac = 10 + r * 2;
                items.push(slint::format!("{ac}").into());
            } else if c == 4 {
                let hp = 100 + r * 10;
                items.push(slint::format!("{hp}").into());
            }
        }

        row_data.push(items.into());
    }


    app.global::<TableViewPageAdapter>().set_row_data(row_data.clone().into());
    app.global::<TableViewPageAdapter>().on_filter_sort_model(filter_sort_model);

    app.run().unwrap();
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