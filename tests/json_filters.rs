use rmic::ParameterType;

use crate::utils::{
    syntectic_filter, task_frame_cube, task_hard_sketch, task_phasecongruence, task_should_fail,
};
mod utils;

#[test]
fn check_randomize() {
    let task = task_phasecongruence().resize(1024, 1024).randomize();
    let f = task.filters.clone();

    if let Some(elem) = f.first() {
        let params = elem.parameters.clone();
        assert_ne!(params[1].default, "45");
        assert_ne!(params[2].default, "1");
        assert_ne!(params[3].default, "50");
        assert_eq!(6, params.len(), "param Count");
    }

    let _ = task.execute();
}

#[test]
fn check_params_value() {
    let task = task_hard_sketch();
    let parameters = task.filters.clone();

    if let Some(elem) = parameters.first() {
        let params = elem.parameters.clone();
        assert_eq!(params[0].param_type, ParameterType::Separator);
        assert_eq!(params[1].default, "300");
        assert_eq!(params[1].param_type, ParameterType::Float);
        assert_eq!(params[2].default, "50");
        assert_eq!(params[2].param_type, ParameterType::Float);
        assert_eq!(params[3].default, "1");
        assert_eq!(params[3].param_type, ParameterType::Float);
        assert_eq!(params[4].default, "0.1");
        assert_eq!(params[4].param_type, ParameterType::Float);
        assert_eq!(params[5].default, "20");
        assert_eq!(params[5].param_type, ParameterType::Float);
        assert_eq!(params[6].default, "0");
        assert_eq!(params[6].param_type, ParameterType::Bool);
        assert_eq!(params[7].default, "4");
        assert_eq!(params[7].param_type, ParameterType::Choice);
        assert_eq!(10, params.len(), "param Count");
    }

    let _ = task.execute();
}

#[test]
fn invalid_json_effect() {
    let result = task_should_fail().execute();
    assert!(result.is_err())
}

#[test]
fn check_filter_parsing() {
    let g = syntectic_filter();
    let u = g.parameters;

    // let task = task_phasecongruence().resize(1024, 1024).randomize();
    // assert!(result.is_err())
}

#[test]
fn parse_choice_position() {
    let g = task_frame_cube();

    let res = g.execute();
    assert!(res.is_ok());

    // let task = task_phasecongruence().resize(1024, 1024).randomize();
    // assert!(result.is_err())
}
