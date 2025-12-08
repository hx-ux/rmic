use rmic::{Gmic, GmicError};

const INPUT_IMAGE: &str = "input.jpg";
const OUTPUT_FOLDER: &str = "tests/out";

#[test]
fn command() {
    let name = "water_params";
    let effect = |gmic: Gmic| gmic.add_command("water", &["100", "1", "45"]);
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());

    let name = "water_default";
    let effect = |gmic: Gmic| gmic.add_command("water", &[]);
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}
#[test]
fn degradations() {
    let name = "light_and_cracks";
    let effect = |gmic: Gmic| gmic.add_command("light_patch", &["500", "0.9", "1.7"]).add_command("cracks", &[]);
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}

#[test]
fn raw() {
    let name = "raw_1";
    let effect =
        |gmic: Gmic| gmic.add_raw_arg("polaroid 5,30 rotate 20 drop_shadow , drgba glow 10%");
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());

    let name = "raw_stacked";
    let effect = |gmic: Gmic| {
        gmic.add_raw_arg("polaroid 5,30")
            .add_raw_arg("rotate 20")
            .add_raw_arg("drop_shadow ,")
            .add_raw_arg("drgba glow 10%")
    };
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}

#[test]
fn utils() {
    let name = "50x50";
    let effect = |gmic: Gmic| gmic.resize(50, 50).brightness(1.0);
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());

    let name = "utils_2";
    let effect_chain = |g: Gmic| g.blur(5.0).rotate(90).solarize();
    let result = process_images(name, effect_chain);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}
#[test]

fn draw() {
    let name = "plasma";
    let effect = |gmic: Gmic| gmic.add_raw_arg("400,400,1,3 +plasma 1");
    let result = process_images(name, effect);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());

    let name = "upscale";
    let effect_chain = |g: Gmic| g.add_raw_arg("+noise_hurl[-1] ,");
    let result = process_images(name, effect_chain);
    assert!(result.is_ok(), "G'MIC execution failed: {:?}", result.err());
}

fn process_images<F>(output_file: &str, effect: F) -> Result<(), GmicError>
where
    F: FnOnce(Gmic) -> Gmic,
{
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, output_file);
    let gmic_task = effect(Gmic::new().input(INPUT_IMAGE)).output(_out);
    let result = gmic_task.execute();
    result
}
