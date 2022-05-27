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
    Italic,
    Bold,
}

#[derive(Debug, Clone)]
pub enum TemplateElement {
    Header(u32, Vec<TemplateElement>),
    Paragraph(Vec<TemplateElement>),
    Line(Vec<TemplateElement>),
    Text(String),
    Italic(Vec<TemplateElement>),
    Bold(Vec<TemplateElement>),
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
                (
                    TemplateElementType::Italic,
                    TemplateString::parse_string("<i>{content}</i>"),
                ),
                (
                    TemplateElementType::Bold,
                    TemplateString::parse_string("<b>{content}</b>"),
                ),
            ]),
        }
    }
}

impl TemplateElementType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "header" => Some(TemplateElementType::Header),
            "paragraph" => Some(TemplateElementType::Paragraph),
            "line" => Some(TemplateElementType::Line),
            "text" => Some(TemplateElementType::Text),
            "italic" => Some(TemplateElementType::Italic),
            "bold" => Some(TemplateElementType::Bold),
            _ => None,
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
            TemplateElement::Italic(_) => TemplateElementType::Italic,
            TemplateElement::Bold(_) => TemplateElementType::Bold,
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
            TemplateElement::Line(elements)
            | TemplateElement::Italic(elements)
            | TemplateElement::Bold(elements) => {
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
        TemplateElementType::from_str(file_stem).map(|element_type| {
            self.templates.insert(
                element_type,
                TemplateString::parse_string(&fs::read_to_string(path).unwrap()),
            )
        });
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
                TemplateElementType::Italic => "Italic",
                TemplateElementType::Bold => "Bold",
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
        assert_eq!(
            TemplateElement::Italic(vec![]).to_type(),
            TemplateElementType::Italic
        );
        assert_eq!(
            TemplateElement::Bold(vec![]).to_type(),
            TemplateElementType::Bold
        );
    }

    #[test]
    fn test_template_element_type_display() {
        assert_eq!(format!("{}", TemplateElementType::Header), "Header");
        assert_eq!(format!("{}", TemplateElementType::Paragraph), "Paragraph");
        assert_eq!(format!("{}", TemplateElementType::Line), "Line");
        assert_eq!(format!("{}", TemplateElementType::Text), "Text");
        assert_eq!(format!("{}", TemplateElementType::Italic), "Italic");
        assert_eq!(format!("{}", TemplateElementType::Bold), "Bold");
    }
}
