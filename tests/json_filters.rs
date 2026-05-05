use rmic::{Gmic, GmicError};

use crate::utils::process_images;
mod utils;

#[test]
fn chained_json() {
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
    assert!(result.is_ok());
}

#[test]
fn random_color() {
    let name = "afre_montagex";
    let effect = |gmic: Gmic| {
        gmic.add_json_effect(r#"{
            "name": "Montage X", "lang": "en", "command": "afre_montagex", "command_preview": "afre_montagex_preview", "parameters": [
              { "type": "note", "text": "<strong>Generate montage without resizing.  Filter by <a href=\"https://discuss.pixls.us/u/afre\">afre</a> 2021 Apr16.</strong>\n\n - Set <strong>Input layers</strong> to <strong>All</strong>.\n\n" },
              { "type": "int", "name": "Max Per Row", "default": "5", "min": "1", "max": "25", "pos": "1" },
              { "type": "int", "name": "Spacing", "default": "1", "min": "0", "max": "10", "pos": "2" },
              { "type": "color", "name": "Matte Colour", "default": "230,255,230", "pos": "3" }
              ]
            },"#)
       .resize(1024, 1024).randomize()
    };

    let result = process_images(name, effect);
    assert!(result.is_ok());
}
