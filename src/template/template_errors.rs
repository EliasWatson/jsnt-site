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
