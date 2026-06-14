use crate::utils::{task_frame_cube, task_hard_sketch, task_phasecongruence, task_should_fail};
use rmic::{Filter, Parameter, ParameterChoice, ParameterType};
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
        assert_eq!(4, params.len(), "param Count");
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
fn parse_choice_position() {
    if let Some(filter) = task_frame_cube().filters.first() {
        let params = &filter.parameters;
        if let Some(choice_param) = params
            .iter()
            .find(|p| p.param_type == ParameterType::Choice)
        {
            assert_eq!(
                choice_param.choices,
                Some(vec![
                    ParameterChoice {
                        value: "0".to_string(),
                        label: "Normal".to_string(),
                    },
                    ParameterChoice {
                        value: "1".to_string(),
                        label: "Mirror-X".to_string(),
                    },
                    ParameterChoice {
                        value: "2".to_string(),
                        label: "Mirror-Y".to_string(),
                    },
                    ParameterChoice {
                        value: "3".to_string(),
                        label: "Mirror-XY".to_string(),
                    },
                ])
            );
            assert_eq!(choice_param.default, "0");
        }
    }
}

#[test]
fn test_cli_value_and_forargs() {
    let params = vec![
        Parameter::new(ParameterType::Text, "hello", None, None, 0),
        Parameter::new(ParameterType::Value, "world", None, None, 1),
        Parameter::new(ParameterType::Int, "42", None, None, 2),
        Parameter::new(ParameterType::Text, "\"already\"", None, None, 3),
    ];

    let filter = Filter::new("test_filter".to_string(), params);

    assert_eq!(filter.parameters[0].cli_value(), "\"hello\"");
    assert_eq!(filter.parameters[1].cli_value(), "\"world\"");
    assert_eq!(filter.parameters[2].cli_value(), "42");
    assert_eq!(filter.parameters[3].cli_value(), "\"already\"");

    let args = filter.to_cli_command();
    assert_eq!(
        args,
        vec![
            "test_filter".to_string(),
            "\"hello\",\"world\",42,\"already\"".to_string(),
        ]
    );
}
