use rmic::{Gmic, GmicError};

use crate::utils::process_images;
mod utils;

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
    let effect = |gmic: Gmic| gmic.add_raw_arg("fx_maze 24,8,0,1,0").resize(500, 500);
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn whirly_lines() {
    let name = "whirly_lines";
    let effect =
        |gmic: Gmic| gmic.add_raw_arg("fx_whirling_lines 30,30,0,3,3,6,0,0,0.45,40,60,0,0");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn diffusion() {
    let name = "diffusion";
    let effect = |gmic: Gmic| gmic.add_raw_arg("fx_diffusiontensors 20,16,2,2,0.15,1,0,3");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn break_mirror() {
    let name = "fx_breaks";
    let effect = |gmic: Gmic| gmic.add_raw_arg("fx_breaks 1,212.7,30,10,3");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn reflect() {
    let name = "reflect";
    let effect = |gmic: Gmic| gmic.add_raw_arg("fx_reflect 50,1,110,160,190,64,0,1.5,0,-3.3,7,1.5");
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn benchmark() {
    let name = "benchmark";
    let effect = |gmic: Gmic| {
        gmic.add_raw_arg("fx_breaks 1,212.7,30,10,3")
            .add_raw_arg("fx_reflect 50,1,110,160,190,64,0,1.5,0,-3.3,7,1.5")
            .add_raw_arg("fx_diffusiontensors 20,16,2,2,0.15,1,0,3")
            .solarize()
            .blur(0.1)
            .resize(1024, 1024)
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}

#[test]
fn big_chaos() {
    let name = "chaos";
    let effect = |gmic: Gmic| {
        gmic.add_raw_arg("fx_ultrawarp4plus 0,0,3.3,0,0,5,0,0,0,4,256,4.8,5,2,0,0,2,3,3,20,1,1,0,5,2,1,2,0.25,1,1,0,5,0,3,0.5,2,-180,0,0,1,11,0,0")
            .resize(1024, 1024)
    };
    let result = process_images(name, effect);
    assert!(result.is_ok());
}
