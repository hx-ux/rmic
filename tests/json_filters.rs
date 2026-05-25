use crate::utils::{task_hard_sketch, task_phasecongruence, task_should_fail};
mod utils;

#[test]
fn check_randomize() {
    let task = task_phasecongruence().resize(1024, 1024).randomize();
    let f = task.filters.clone();

    if let Some(elem) = f.first() {
        let params = elem.parameters.clone();
        assert_ne!(params[0].default, "45");
        assert_ne!(params[1].default, "1");
        assert_ne!(params[2].default, "50");
        assert_eq!(4, params.len(), "param Count");
    }

    let _ = task.execute();
}

#[test]
fn check_params() {
    let task = task_hard_sketch();
    let parameters = task.filters.clone();

    if let Some(elem) = parameters.first() {
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

    let _ = task.execute();
}

#[test]
fn invalid_json_effect() {
    let result = task_should_fail().execute();
    assert!(result.is_err())
}
