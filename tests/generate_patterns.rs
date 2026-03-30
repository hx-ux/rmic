use rmic::{Gmic, GmicError};
const INPUT_IMAGE: &str = "input.jpg";
const OUTPUT_FOLDER: &str = "tests/out";

#[test]
fn trippy_pattern() {
    let name = "trippy";
    let effect = |gmic: Gmic| gmic.add_raw_arg("jeje_freqy_pattern 39.4,33,55.55,0");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn maze() {
    let name = "maze";
    let effect = |gmic: Gmic| gmic.add_raw_arg("fx_maze 24,8,0,1,0");
    let result = process_images(name, effect);
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
