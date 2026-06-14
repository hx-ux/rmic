use std::path::PathBuf;

use crate::utils::{INPUT_IMAGE, OUTPUT_FOLDER};
use rand::seq::IndexedRandom;
use rmic::{Filter, FilterList, Gmic};
mod utils;

fn load_filter_list() -> Option<FilterList> {
    let mut path = PathBuf::new();
    path.push("tests/assets/update376.json");
    if let Ok(v) = FilterList::load_local(&path) {
        Some(v)
    } else {
        None
    }
}

fn get_random_filters(count: i8) -> Vec<Filter> {
    let g = load_filter_list();
    let mut selected = Vec::new();

    match g {
        Some(ff) => {
            for _ in 0..count {
                let mut rng_cat = rand::rng();
                let mut rng_filter = rand::rng();
                if let Some(category) = ff.categories.choose(&mut rng_cat) {
                    if let Some(filter) = category.filters.choose(&mut rng_filter) {
                        selected.push(filter.clone());
                    }
                }
            }
            selected
        }
        None => selected,
    }
}

#[test]
fn random_filter_from_list() {
    let filter_count = 3;
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, "random");
    let filters = get_random_filters(filter_count);

    let gmic_task = Gmic::new()
        .input(INPUT_IMAGE)
        .add_object_filters(filters.clone())
        .output(_out)
        .randomize();

    let summary = gmic_task.summary();

    let summary_path = format!("{}/summary.txt", OUTPUT_FOLDER);
    std::fs::write(&summary_path, summary.as_bytes()).expect("Failed to write summary file");

    let task = gmic_task.execute();

    assert!(task.is_ok());
}
