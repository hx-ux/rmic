use rmic::{Gmic, GmicError};

const INPUT_IMAGE: &str = "input.jpg";
const OUTPUT_FOLDER: &str = "tests/out";

#[test]
fn wildcard() {
    let name = "wildcard";
    let effect =
        |gmic: Gmic| gmic.add_raw_arg("fx_whirling_lines 30,30,0,3,3,6,0,0,0.45,40,60,0,0");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn command_no_params() {
    let name = "water_default";
    let effect = |gmic: Gmic| gmic.add_command("water", &[]);
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn command_params() {
    let name = "water_params";
    let effect = |gmic: Gmic| gmic.add_command("water", &["100", "1", "45"]);
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn commands_stacked() {
    let name = "light_and_cracks";
    let effect = |gmic: Gmic| {
        gmic.add_command("light_patch", &["500", "0.9", "1.7"])
            .add_command("cracks", &[])
            .add_command("water", &[])
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn raw() {
    let name = "raw_one_line";
    let effect =
        |gmic: Gmic| gmic.add_raw_arg("polaroid 5,30 rotate 20 drop_shadow , drgba glow 10%");
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}

#[test]
fn raw_stacked() {
    let name = "raw_stacked";
    let effect = |gmic: Gmic| {
        gmic.add_raw_arg("polaroid 5,30")
            .add_raw_arg("rotate 20")
            .add_raw_arg("drop_shadow ,")
            .add_raw_arg("drgba glow 10%")
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn resize() {
    let name = "50x50";
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

fn process_images<F>(output_file: &str, effect: F) -> Result<(), GmicError>
where
    F: FnOnce(Gmic) -> Gmic,
{
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, output_file);
    let gmic_task = effect(Gmic::new().input(INPUT_IMAGE)).output(_out);
    gmic_task.execute()
}
