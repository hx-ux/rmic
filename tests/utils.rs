use rmic::{Gmic, GmicError};

const INPUT_IMAGE: &str = "input.jpg";
const OUTPUT_FOLDER: &str = "tests/out";

pub fn process_images<F>(output_file: &str, effect: F) -> Result<String, GmicError>
where
    F: FnOnce(Gmic) -> Gmic,
{
    let _out = format!("{}/{}.jpg", OUTPUT_FOLDER, output_file);
    let gmic_task = effect(Gmic::new().input(INPUT_IMAGE)).output(_out);
    let command = gmic_task.dry_run();

    gmic_task.execute().map(|_| command)
}
