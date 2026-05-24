use rmic::{Gmic, GmicError};

use crate::utils::{INPUT_IMAGE, OUTPUT_FOLDER, process_images};
mod utils;

#[test]
fn check_randomize() {
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, "phase");

    let gmic_task =
        Gmic::new().input(INPUT_IMAGE).add_json_effect(r#" {
         "name": "Jpr Phasecongruence", "lang": "en", "command": "jpr_phasecongruence", "command_preview": "jpr_phasecongruence", "parameters": [
         { "type": "note", "text": "Edge detect with directional Phase Congruence using proportionality to Local Energy." },
         { "type": "float", "name": "Start Angle", "default": "45", "min": "0", "max": "360", "pos": "1" },
         { "type": "int", "name": "Directions", "default": "1", "min": "1", "max": "20", "pos": "2" },
         { "type": "float", "name": "Energy Threshold", "default": "50", "min": "0", "max": "500", "pos": "3" },
         { "type": "bool", "name": "Local Maxima", "default": "1", "pos": "4" },
         { "type": "note", "text": "update 2013-Mar-31 author @jayprich" }
         ]
       }"#)
       .resize(1024, 1024).randomize().output(_out);

    let p = gmic_task.filters.clone();

    if let Some(elem) = p.first() {
        let params = elem.parameters.clone();
        assert_ne!(params[0].default, "45");
        assert_ne!(params[1].default, "1");
        assert_ne!(params[2].default, "50");
        assert_eq!(4, params.len(), "param Count");
    }

    let _ = gmic_task.execute();
}

#[test]
fn check_params() {
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, "sketch");

    let gmic_task =
        Gmic::new().input(INPUT_IMAGE).add_json_effect(r#"{
        "name": "Hard Sketch", "lang": "en", "command": "fx_hardsketchbw", "command_preview": "fx_hardsketchbw", "parameters": [
        { "type": "separator" },
        { "type": "float", "name": "Amplitude", "default": "300", "min": "0", "max": "4000", "pos": "1" },
        { "type": "float", "name": "Density", "default": "50", "min": "0", "max": "100", "pos": "2" },
        { "type": "float", "name": "Smoothness", "default": "1", "min": "0", "max": "10", "pos": "3" },
        { "type": "float", "name": "Opacity", "default": "0.1", "min": "0", "max": "1", "pos": "4" },
        { "type": "float", "name": "Edge", "default": "20", "min": "0", "max": "100", "pos": "5" },
        { "type": "bool", "name": "Fast Approximation", "default": "0", "pos": "6" },
        { "type": "choice", "name": "Color Model", "default": "4", "pos": "7", "choices": { "0": "Black on white", "1": "White on black", "2": "Black on transparent white", "3": "White on transparent black", "4": "Color on white" } },
        { "type": "separator" },
        { "type": "note", "text": "Author: David Tschumperlé.      Latest Update: 2010/12/29." }
        ]
        }"#).output(_out);

    let p = gmic_task.filters.clone();

    if let Some(elem) = p.first() {
        let params = elem.parameters.clone();
        assert_eq!(params[0].default, "300");
        assert_eq!(params[1].default, "50");
        assert_eq!(params[2].default, "1");
        assert_eq!(params[3].default, "0.1");
        assert_eq!(params[4].default, "20");
        assert_eq!(params[5].default, "0");
        assert_eq!(params[6].default, "4");
        assert_eq!(7, params.len(), "param Count");
    }

    let _ = gmic_task.execute();
}

#[test]
fn invalid_json_effect() {
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, "linear_gradient");

    let gmic_task = Gmic::new()
        .input(INPUT_IMAGE)
        .add_json_effect(
            r#"{
            "name": "Plasma",
            "lang": "en",
            "command": "fx_plasma",
            "command_preview": "fx_plasma",
            "parameters": [
                "type": "int",
                "name": "Scale",
                "default": "8",
                "min": "2",
                "max": "10",
                "pos": "3"
              },
              { "type": "bool", "name": "Randomize", "default": "0", "pos": "4" },
              {
                "type": "bool",
                "name": "Transparency",
                "default": "0",
                "pos": "5"
              },
              { "type": "separator" },
              {
                "type": "note",
                "text": "Author: David Tschumperlé.      Latest Update: 2011/03/20."
              }
            ]
          }"#,
        )
        .output(_out);

    let r = gmic_task.execute();
    assert!(r.is_err())
}
