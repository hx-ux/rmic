use crate::utils::process_images;
use rmic::Gmic;
mod utils;

#[test]
fn command_params() {
    let name = "water_params";
    let effect = |gmic: Gmic| gmic.add_effect("water", &["100", "1", "45"]);
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn commands_stacked() {
    let name = "card";
    let effect = |gmic: Gmic| {
        gmic.add_effect("light_patch", &["500", "0.9", "1.7"])
            .add_effect("glow", &["10%"])
            .add_raw_effect("polaroid 5,30")
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn raw() {
    let name = "raw_one_line";
    let effect =
        |gmic: Gmic| gmic.add_raw_effect("polaroid 5,30 rotate 20 drop_shadow , drgba glow 10%");
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}

#[test]
fn raw_stacked() {
    let name = "raw_stacked";
    let effect = |gmic: Gmic| {
        gmic.add_raw_effect("polaroid 5,30")
            .add_raw_effect("rotate 20")
            .add_raw_effect("drop_shadow ,")
            .add_raw_effect("drgba glow 10%")
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn resize() {
    let name = "resized_50x50";
    let effect = |gmic: Gmic| gmic.resize(50, 50).brightness(1.0);
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn utils_collection() {
    let name = "blur_rotate_solatize";
    let effect_chain = |g: Gmic| g.blur(5.0).rotate(90).solarize();
    let result = process_images(name, effect_chain);
    assert!(result.is_ok());
}

