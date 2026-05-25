use rmic::{Gmic, GmicError};

use crate::utils::process_images;
mod utils;
#[test]
fn check_dryrun() {
    let name = "water_params";
    let effect = |gmic: Gmic| gmic.add_raw_filter("polaroid 5,30");
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
