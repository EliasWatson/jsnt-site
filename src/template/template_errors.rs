use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TemplateError {
    MissingVariables(HashSet<String>),
    MissingTemplate(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::MissingVariables(missing_variables) => {
                let variable_list = missing_variables
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "contents not set for variables: {}", variable_list)
            }
            TemplateError::MissingTemplate(template_name) => {
                write!(f, "missing template for {}", template_name)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::template::template_errors::TemplateError;
    use std::collections::HashSet;

    #[test]
    fn test_missing_variables() {
        let err_1 = TemplateError::MissingVariables(HashSet::from(["variable_name".to_string()]));
        assert_eq!(
            format!("{}", err_1),
            "contents not set for variables: variable_name"
        );
        assert_eq!(
            err_1,
            TemplateError::MissingVariables(HashSet::from(["variable_name".to_string()]))
        );
        assert_ne!(
            err_1,
            TemplateError::MissingVariables(HashSet::from(["variable-name".to_string()]))
        );
        assert_eq!(
            err_1.clone(),
            TemplateError::MissingVariables(HashSet::from(["variable_name".to_string()]))
        );

        let err_2 = TemplateError::MissingVariables(HashSet::from([
            "var1".to_string(),
            "var2".to_string(),
        ]));
        let possible_outputs_2 = [
            "contents not set for variables: var1, var2",
            "contents not set for variables: var2, var1",
        ];
        assert!(possible_outputs_2.contains(&format!("{}", err_2).as_str()));
        assert_eq!(
            err_2,
            TemplateError::MissingVariables(HashSet::from([
                "var1".to_string(),
                "var2".to_string()
            ]))
        );
        assert_ne!(
            err_2,
            TemplateError::MissingVariables(HashSet::from([
                "var1".to_string(),
                "var3".to_string()
            ]))
        );
        assert_eq!(
            err_2.clone(),
            TemplateError::MissingVariables(HashSet::from([
                "var1".to_string(),
                "var2".to_string()
            ]))
        );
    }

    #[test]
    fn test_missing_template() {
        let err = TemplateError::MissingTemplate("example_template".to_string());
        assert_eq!(format!("{}", err), "missing template for example_template");
        assert_eq!(
            err,
            TemplateError::MissingTemplate("example_template".to_string())
        );
        assert_ne!(
            err,
            TemplateError::MissingTemplate("example-template".to_string())
        );
        assert_eq!(
            err.clone(),
            TemplateError::MissingTemplate("example_template".to_string())
        );
    }
}
