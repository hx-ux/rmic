use rmic::{Gmic, GmicError};

pub const INPUT_IMAGE: &str = "input.jpg";
pub const OUTPUT_FOLDER: &str = "tests/out";

pub fn process_images<F>(output_file: &str, effect: F) -> Result<String, GmicError>
where
    F: FnOnce(Gmic) -> Gmic,
{
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, output_file);
    let gmic_task = effect(Gmic::new().input(INPUT_IMAGE)).output(_out);
    let command = gmic_task.dry_run();

    gmic_task.execute().map(|_| command)
}

pub fn task_phasecongruence() -> Gmic {
    let out = format!("{}/{}.jpg", OUTPUT_FOLDER, "phasecongruence");
    let gmic_task =
        Gmic::new().input(INPUT_IMAGE).add_json_filter(r#" {
         "name": "Jpr Phasecongruence", "lang": "en", "command": "jpr_phasecongruence", "command_preview": "jpr_phasecongruence", "parameters": [
         { "type": "note", "text": "Edge detect with directional Phase Congruence using proportionality to Local Energy." },
         { "type": "float", "name": "Start Angle", "default": "45", "min": "0", "max": "360", "pos": "1" },
         { "type": "int", "name": "Directions", "default": "1", "min": "1", "max": "20", "pos": "2" },
         { "type": "float", "name": "Energy Threshold", "default": "50", "min": "0", "max": "500", "pos": "3" },
         { "type": "bool", "name": "Local Maxima", "default": "1", "pos": "4" },
         { "type": "note", "text": "update 2013-Mar-31 author @jayprich" }
         ]
       }"#).output(out);

    gmic_task
}

pub fn task_hard_sketch() -> Gmic {
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, "hard_sketch");

    let gmic_task =
        Gmic::new().input(INPUT_IMAGE).add_json_filter(r#"{
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

    gmic_task
}

pub fn task_should_fail() -> Gmic {
    Gmic::new()
        .input(INPUT_IMAGE)
        .add_json_filter(
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
          }"#,
        )
        .output("none")
}
