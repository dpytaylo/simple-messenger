use std::borrow::Cow;

use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FlattenValidationError {
    pub name: Vec<&'static str>,
    pub code: Cow<'static, str>,
    pub message: Option<Cow<'static, str>>,
}

pub fn flatten(errors: ValidationErrors) -> Vec<FlattenValidationError> {
    let mut buffer = Vec::with_capacity(errors.errors().len());
    let mut stack = vec![(Vec::new(), errors.errors().iter())];

    'outer: while !stack.is_empty() {
        let last_idx = stack.len() - 1;

        while let Some((&name, kind)) = stack[last_idx].1.next() {
            let mut path = stack[last_idx].0.clone();
            path.push(name);

            match kind {
                ValidationErrorsKind::Struct(err) => {
                    stack.push((path, err.errors().iter()));
                    continue 'outer;
                }

                ValidationErrorsKind::List(errors) => {
                    stack.extend(
                        errors
                            .iter()
                            .map(|(_, err)| (path.clone(), err.errors().iter())),
                    );
                    continue 'outer;
                }

                ValidationErrorsKind::Field(errors) => {
                    for error in errors {
                        buffer.push(FlattenValidationError {
                            name: path.clone(),
                            code: error.code.clone(),
                            message: error.message.clone(),
                        });
                    }
                }
            }
        }

        stack.remove(last_idx);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
    };

    use validator::ValidationError;

    use super::*;

    #[test]
    fn test_to_user_string() {
        let mut errors = ValidationErrors::new();

        errors.add(
            "field1",
            ValidationError {
                code: Cow::from("code1"),
                message: None,
                params: HashMap::new(),
            },
        );

        let mut struct_errors = ValidationErrors::new();
        struct_errors.add("struct_field", ValidationError::new("struct_code"));
        errors.errors_mut().insert(
            "struct",
            ValidationErrorsKind::Struct(Box::new(struct_errors)),
        );

        let mut map = BTreeMap::new();

        let mut v_errors = ValidationErrors::new();
        v_errors.add("map_field1", ValidationError::new("map_code1"));
        v_errors.add("map_field2", ValidationError::new("map_code2"));
        map.insert(0, Box::new(v_errors));

        let mut v_errors = ValidationErrors::new();
        v_errors.add("map_field3", ValidationError::new("map_code3"));
        map.insert(1, Box::new(v_errors));

        errors
            .errors_mut()
            .insert("list", ValidationErrorsKind::List(map));

        errors.add(
            "field2",
            ValidationError {
                code: "code2".into(),
                message: Some("message".into()),
                params: HashMap::new(),
            },
        );

        let mut sorted = flatten(errors);
        sorted.sort_unstable();

        assert_eq!(
            sorted,
            [
                FlattenValidationError {
                    name: vec!["field1"],
                    code: "code1".into(),
                    message: None,
                },
                FlattenValidationError {
                    name: vec!["field2"],
                    code: "code2".into(),
                    message: Some("message".into()),
                },
                FlattenValidationError {
                    name: vec!["list", "map_field1"],
                    code: "map_code1".into(),
                    message: None
                },
                FlattenValidationError {
                    name: vec!["list", "map_field2"],
                    code: "map_code2".into(),
                    message: None
                },
                FlattenValidationError {
                    name: vec!["list", "map_field3"],
                    code: "map_code3".into(),
                    message: None
                },
                FlattenValidationError {
                    name: vec!["struct", "struct_field"],
                    code: "struct_code".into(),
                    message: None
                }
            ],
        );
    }
}
