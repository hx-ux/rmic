use crate::utils::{INPUT_IMAGE, OUTPUT_FOLDER};
use rand::RngExt;
use rand::seq::IndexedRandom;
use rmic::{Catalouge, Gmic};
use rmic::{Filter, Parameter, ParameterType};
use std::path::PathBuf;
mod utils;

const EXCLUDE_CATEGORIES: [&str; 6] = [
    "Various",
    "Telperion",
    "McCap",
    "Tom Keil",
    "Layers",
    "About",
];

fn load_catalouge() -> Option<Catalouge> {
    let path = PathBuf::from("tests/assets/update401.json");
    let mut c = Catalouge::load_local(&path).ok()?;

    let exclude_commands = vec![
        "fx_blend".to_string(),
        "fx_transfer_pca".to_string(),
        "fx_hue_overlay_masks".to_string(),
        "fx_apply_multiscale".to_string(),
        "fx_compose_vivid_color".to_string(),
        // needs two layers
        "fx_stylize".to_string(),
        // needs two layers
        "fx_MorphoPaint".to_string(),
        // needs two layers
        "fx_clut_from_ab".to_string(),
        // Prop Reference Paralell is faulty
        "albers_projection".to_string(),
    ];
    let exclude_types = vec![ParameterType::File, ParameterType::Button];

    c.categories
        .retain(|cat| !EXCLUDE_CATEGORIES.contains(&cat.name.as_str()));

    for cat in &mut c.categories {
        cat.filters.retain(|f| {
            !f.has_property(Some(exclude_commands.clone()), Some(exclude_types.clone()))
        });
    }

    Some(c)
}

#[test]
fn can_filter_list_be_loaded() {
    if let Some(c) = load_catalouge() {
        assert!(c.categories.len() == 68);
    }
}

#[test]
fn filter_catalouge() {
    let exclude_commands = vec![
        "fx_blend".to_string(),
        "fx_transfer_pca".to_string(),
        "fx_hue_overlay_masks".to_string(),
        "fx_apply_multiscale".to_string(),
        "fx_compose_vivid_color".to_string(),
        // needs two layers
        "fx_stylize".to_string(),
        // needs two layers
        "fx_MorphoPaint".to_string(),
    ];
    if let Some(c) = load_catalouge() {
        for cat in c.categories {
            for f in cat.filters {
                if f.has_property(Some(exclude_commands.clone()), None) {}
            }
        }
    }
}

fn get_random_filters_in_category(count: i8, name: &str) -> Vec<Filter> {
    let mut selected = Vec::new();
    let mut rng = rand::rng();

    if let Some(mut catalogue) = load_catalouge() {
        catalogue.categories.retain(|f| f.name == name);

        if let Some(category) = catalogue.categories.first() {
            for _ in 0..count {
                if let Some(filter) = category.filters.choose(&mut rng) {
                    selected.push(filter.clone());
                }
            }
        }
    }
    selected
}

fn get_random_filters(count: i8) -> Vec<Filter> {
    let mut selected = Vec::new();

    match load_catalouge() {
        Some(catalouge) => {
            for _ in 0..count {
                let mut rng_cat = rand::rng();
                let mut rng_filter = rand::rng();

                if let Some(category) = catalouge.categories.choose(&mut rng_cat) {
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
fn render_certain_filter() {
    if let Some(l) = load_catalouge() {
        let result = l.find_filter("albers_projection").cloned();
        if let Some(g) = result {
            let gmic_task = Gmic::new()
                .input(INPUT_IMAGE)
                .add_object_filters(vec![g.clone()])
                .output(format!(
                    "{}/{}.jpg",
                    OUTPUT_FOLDER,
                    g.command.clone().to_string()
                ))
                .randomize();

            let r = gmic_task.execute();
            assert!(r.is_ok());
        }
    }
}

#[test]
fn random_filter_from_list() {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();

    let filter_count = 5;
    //let filters = get_random_filters(filter_count);
    let filters_cat = get_random_filters_in_category(filter_count, "Artistic");

    let gmic_task = Gmic::new()
        .input(INPUT_IMAGE)
        .add_object_filters(filters_cat.clone())
        .output(format!(
            "{}/{}_{}.jpg",
            OUTPUT_FOLDER, "random", random_number
        ))
        .randomize();

    let summary = gmic_task.summary();

    let summary_path = format!("{}/summary_{}.txt", OUTPUT_FOLDER, random_number);
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

#[test]
fn find_certain_filter() {
    let mut result: Option<Filter> = None;

    if let Some(l) = load_catalouge() {
        result = l.find_filter("query").cloned();
    }
    assert!(result.is_none());
}
