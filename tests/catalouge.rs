use crate::utils::{INPUT_IMAGE, OUTPUT_FOLDER};
use rand::seq::IndexedRandom;
use rmic::{Catalouge, Gmic};
use rmic::{Filter, Parameter, ParameterType};
use std::path::PathBuf;
mod utils;

fn load_filter_list() -> Option<Catalouge> {
    let mut path = PathBuf::new();
    path.push("tests/assets/update376.json");
    match Catalouge::load_local(&path) {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

#[test]
fn can_filter_list_be_loaded() {
    if let Some(c) = load_filter_list() {
        assert!(c.categories.len() == 68);
    }
}

#[test]
fn filter_catalouge() {
    if let Some(c) = load_filter_list() {}
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

#[test]
fn exclude_by_command() {
    let filter = Filter::new_params("fx_blend".to_string(), vec![]);

    let exclude = vec!["fx_blend".to_string(), "fx_transfer_pca".to_string()];
    assert!(filter.has_property(Some(exclude), None));
}

#[test]
fn exclude_by_param_type() {
    let exclude = vec![ParameterType::Note, ParameterType::Separator];

    let filter = Filter::new_params(
        "fx_blend".to_string(),
        vec![Parameter::new(ParameterType::Note, "None", None, None, 0)],
    );

    assert!(filter.has_property(None, Some(exclude)));
}
