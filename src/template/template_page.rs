use crate::template::template_element::{
    render_element_list, TemplateElement, TemplateElementTemplates, TemplateElementType,
};
use crate::template::template_errors::TemplateError;
use crate::template::template_string::TemplateString;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TemplatePage {
    id: String,
    templates: TemplateElementTemplates,
    sections: Vec<TemplatePageSection>,
}

#[derive(Debug, Clone)]
enum TemplatePageSection {
    Text(String),
    Elements(String, Vec<TemplateElement>),
    Variable(String),
    DefaultVariable(String, String),
}

impl TemplatePage {
    pub fn add(&mut self, name: &str, element: TemplateElement) {
        for section in self.sections.iter_mut() {
            match section {
                TemplatePageSection::Variable(variable_name)
                | TemplatePageSection::DefaultVariable(variable_name, _)
                | TemplatePageSection::Elements(variable_name, _)
                    if variable_name == name => {}
                _ => continue,
            };

            if let TemplatePageSection::Elements(_, elements) = section {
                elements.push(element);
            } else {
                *section = TemplatePageSection::Elements(String::from(name), vec![element]);
            }

            break;
        }
    }

    pub fn render(&self) -> Result<String, TemplateError> {
        let mut rendered: Vec<String> = vec![];

        for section in &self.sections {
            match section {
                TemplatePageSection::Text(text) => {
                    rendered.push(text.clone());
                }
                TemplatePageSection::Elements(_, elements) => {
                    rendered.push(render_element_list(elements, &self.templates, "", false)?);
                }
                TemplatePageSection::DefaultVariable(_, default_text) => {
                    rendered.push(default_text.clone());
                }
                TemplatePageSection::Variable(name) => {
                    return Err(TemplateError::MissingVariables(HashSet::from([
                        name.clone()
                    ])));
                }
            };
        }

        return Ok(rendered.join(""));
    }
}
