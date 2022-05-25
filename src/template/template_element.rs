use crate::template::template_errors::TemplateError;
use crate::template::template_string::TemplateString;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TemplateElementType {
    Header,
    Paragraph,
    Line,
    Text,
}

#[derive(Debug, Clone)]
pub enum TemplateElement {
    Header(u32, Vec<TemplateElement>),
    Paragraph(Vec<TemplateElement>),
    Line(Vec<TemplateElement>),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct TemplateElementTemplates {
    templates: HashMap<TemplateElementType, TemplateString>,
}

impl Default for TemplateElementTemplates {
    fn default() -> Self {
        TemplateElementTemplates {
            templates: HashMap::from([
                (
                    TemplateElementType::Header,
                    TemplateString::parse_string("<h{level}>{content}</h{level}>"),
                ),
                (
                    TemplateElementType::Paragraph,
                    TemplateString::parse_string("<p>{content}</p>"),
                ),
                (
                    TemplateElementType::Line,
                    TemplateString::parse_string("{content}"),
                ),
                (
                    TemplateElementType::Text,
                    TemplateString::parse_string("{content}"),
                ),
            ]),
        }
    }
}

impl TemplateElement {
    pub fn to_type(&self) -> TemplateElementType {
        match self {
            TemplateElement::Header(_, _) => TemplateElementType::Header,
            TemplateElement::Paragraph(_) => TemplateElementType::Paragraph,
            TemplateElement::Line(_) => TemplateElementType::Line,
            TemplateElement::Text(_) => TemplateElementType::Text,
        }
    }

    pub fn render(&self, templates: &TemplateElementTemplates) -> Result<String, TemplateError> {
        let mut template = match templates.get(self.to_type()) {
            Some(t) => t.clone(),
            None => return Err(TemplateError::MissingTemplate(self.to_type().to_string())),
        };

        match self {
            TemplateElement::Header(level, elements) => {
                template.set("level", &format!("{}", level));
                template.set(
                    "content",
                    &render_element_list(elements, templates, "", false)?,
                );
            }
            TemplateElement::Paragraph(elements) => {
                template.set(
                    "content",
                    &render_element_list(elements, templates, " ", true)?,
                );
            }
            TemplateElement::Line(elements) => {
                template.set(
                    "content",
                    &render_element_list(elements, templates, "", false)?,
                );
            }
            TemplateElement::Text(text) => {
                template.set("content", text);
            }
        };

        return template.render();
    }
}

pub fn render_element_list(
    elements: &Vec<TemplateElement>,
    templates: &TemplateElementTemplates,
    delimiter: &str,
    trim: bool,
) -> Result<String, TemplateError> {
    let mut rendered = vec![];

    for element in elements {
        rendered.push(element.render(templates)?);
    }

    if trim {
        for s in rendered.iter_mut() {
            *s = s.trim().to_string();
        }
    }

    return Ok(rendered.join(delimiter));
}

impl TemplateElementTemplates {
    pub fn load(&mut self, file_stem: &str, path: &PathBuf) {
        match file_stem {
            "header" => {
                self.templates.insert(
                    TemplateElementType::Header,
                    TemplateString::parse_string(&fs::read_to_string(path).unwrap()),
                );
            }
            "paragraph" => {
                self.templates.insert(
                    TemplateElementType::Paragraph,
                    TemplateString::parse_string(&fs::read_to_string(path).unwrap()),
                );
            }
            "line" => {
                self.templates.insert(
                    TemplateElementType::Line,
                    TemplateString::parse_string(&fs::read_to_string(path).unwrap()),
                );
            }
            "text" => {
                self.templates.insert(
                    TemplateElementType::Text,
                    TemplateString::parse_string(&fs::read_to_string(path).unwrap()),
                );
            }
            _ => {}
        }
    }

    pub fn add(&mut self, element_type: TemplateElementType, template: TemplateString) {
        self.templates.insert(element_type, template);
    }

    pub fn get(&self, element_type: TemplateElementType) -> Option<&TemplateString> {
        self.templates.get(&element_type)
    }
}

impl Display for TemplateElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TemplateElementType::Header => "Header",
                TemplateElementType::Paragraph => "Paragraph",
                TemplateElementType::Line => "Line",
                TemplateElementType::Text => "Text",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use crate::template::template_element::{TemplateElement, TemplateElementType};

    #[test]
    fn test_template_element_to_type() {
        assert_eq!(
            TemplateElement::Header(0, vec![]).to_type(),
            TemplateElementType::Header
        );
        assert_eq!(
            TemplateElement::Paragraph(vec![]).to_type(),
            TemplateElementType::Paragraph
        );
        assert_eq!(
            TemplateElement::Line(vec![]).to_type(),
            TemplateElementType::Line
        );
        assert_eq!(
            TemplateElement::Text(String::new()).to_type(),
            TemplateElementType::Text
        );
    }

    #[test]
    fn test_template_element_type_display() {
        assert_eq!(format!("{}", TemplateElementType::Header), "Header");
        assert_eq!(format!("{}", TemplateElementType::Paragraph), "Paragraph");
        assert_eq!(format!("{}", TemplateElementType::Line), "Line");
        assert_eq!(format!("{}", TemplateElementType::Text), "Text");
    }
}
