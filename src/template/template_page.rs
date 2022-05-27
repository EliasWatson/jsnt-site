use crate::template::template_element::{
    render_element_list, TemplateElement, TemplateElementTemplates,
};
use crate::template::template_errors::TemplateError;
use crate::template::template_string::{TemplateString, TemplateStringSection};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
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
    pub fn load(id: String, path: &PathBuf) -> Self {
        let mut page = TemplatePage::default();
        page.id = id;

        for entry_res in fs::read_dir(path).unwrap() {
            let entry = match entry_res {
                Ok(x) => x,
                Err(_) => continue,
            };

            let metadata = match entry.metadata() {
                Ok(x) => x,
                Err(_) => continue,
            };

            if !metadata.is_file() {
                continue;
            }

            let file_stem = match entry.path().as_path().file_stem() {
                Some(s) => s.to_string_lossy().into_owned(),
                None => continue,
            };

            if &file_stem == "page" {
                page.parse_sections_string(&fs::read_to_string(entry.path()).unwrap());
            } else {
                page.templates.load(&file_stem, &entry.path());
            }
        }

        return page;
    }

    fn parse_sections_string(&mut self, s: &str) {
        // This should be split into separate parser
        let template_string = TemplateString::parse_string(s);
        self.sections = template_string
            .sections
            .iter()
            .map(|section| match section {
                TemplateStringSection::Text(text) => TemplatePageSection::Text(text.clone()),
                TemplateStringSection::Variable(name) => {
                    TemplatePageSection::Variable(name.clone())
                }
                TemplateStringSection::DefaultVariable(name, text) => {
                    TemplatePageSection::DefaultVariable(name.clone(), text.clone())
                }
            })
            .collect();
    }

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
