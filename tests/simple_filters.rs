use crate::utils::process_images;
use rmic::Gmic;
mod utils;

#[test]
fn from_json_string() {
    let name = "CeKoaSa";
    let effect = |gmic: Gmic| {
        gmic.add_json_effect(r#" {
         "name": "Jpr Phasecongruence", "lang": "en", "command": "jpr_phasecongruence", "command_preview": "jpr_phasecongruence", "parameters": [
         { "type": "note", "text": "Edge detect with directional Phase Congruence using proportionality to Local Energy." },
         { "type": "float", "name": "Start Angle", "default": "45", "min": "0", "max": "360", "pos": "1" },
         { "type": "int", "name": "Directions", "default": "1", "min": "1", "max": "20", "pos": "2" },
         { "type": "float", "name": "Energy Threshold", "default": "50", "min": "0", "max": "500", "pos": "3" },
         { "type": "bool", "name": "Local Maxima", "default": "1", "pos": "4" },
         { "type": "note", "text": "update 2013-Mar-31 author @jayprich" }
         ]
       }"#)
       .resize(1024, 1024).randomize()
    };

    let result = process_images(name, effect);

    match result {
        Ok(c) => {
            print!("{} -- ", c);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn check_dryrun() {
    let name = "water_params";
    let effect = |gmic: Gmic| gmic.add_raw_effect("polaroid 5,30");
    let result = process_images(name, effect);

    let mut command = "".to_string();
    if let Ok(c) = result {
        command = c.to_string();
    }
    assert_eq!(
        "-input input.jpg polaroid 5,30 -output tests/out/water_params.jpg".to_string(),
        command
    );
}

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

#[test]
fn random() {
    let name = "blur_rotate_solatize";
    let effect_chain = |g: Gmic| g.add_raw_effect("random_pattern 256");
    let result = process_images(name, effect_chain);
    assert!(result.is_ok());
}

