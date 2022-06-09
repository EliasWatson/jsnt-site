use crate::markdown::document::{Document, Element, InlineElement, Line};
use crate::template::template_element::TemplateElement;
use crate::{markdown, Template};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Pages {
    pages: Vec<Page>,
}

#[derive(Debug)]
struct Page {
    name: String,
    tag: String,
    document: Document,
}

impl Pages {
    pub fn load(path: &PathBuf) -> Self {
        let mut pages = Pages { pages: vec![] };

        pages.load_pages(path);

        return pages;
    }

    fn load_pages(&mut self, path: &PathBuf) {
        for entry_res in fs::read_dir(path).unwrap() {
            let entry = match entry_res {
                Ok(x) => x,
                Err(_) => continue,
            };

            let metadata = match entry.metadata() {
                Ok(x) => x,
                Err(_) => continue,
            };

            if !metadata.is_dir() {
                continue;
            }

            let tag_name = entry.file_name().to_string_lossy().into_owned();

            for entry_res in fs::read_dir(entry.path()).unwrap() {
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

                self.pages.push(Page::load(&tag_name, &entry.path()));
            }
        }
    }

    pub fn render(&self, template: &Template) -> Vec<(String, String)> {
        self.pages
            .iter()
            .map(|page| (page.name.clone(), page.render(template)))
            .collect()
    }
}

impl Page {
    fn load(tag: &str, path: &PathBuf) -> Self {
        Page {
            name: path
                .as_path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            tag: tag.to_string(),
            document: markdown::parser::parse(&fs::read_to_string(path).unwrap()),
        }
    }

    fn render(&self, template: &Template) -> String {
        let mut page_template = template.get_page(&self.tag).unwrap().clone();
        page_template.add("title", TemplateElement::Text("Article".to_string()));

        for element in &self.document.elements {
            page_template.add("content", markdown_element_to_template_element(element));
        }

        return page_template.render().unwrap();
    }
}

fn markdown_element_to_template_element(element: &Element) -> TemplateElement {
    match element {
        Element::Header(level, line) => {
            TemplateElement::Header(*level, markdown_line_to_template_elements(line))
        }
        Element::Paragraph(lines) => TemplateElement::Paragraph(
            lines
                .iter()
                .map(|line| TemplateElement::Line(markdown_line_to_template_elements(line)))
                .collect(),
        ),
    }
}

fn markdown_line_to_template_elements(line: &Line) -> Vec<TemplateElement> {
    line.elements
        .iter()
        .map(|element| markdown_inline_element_to_template_element(element))
        .collect()
}

fn markdown_inline_element_to_template_element(element: &InlineElement) -> TemplateElement {
    match element {
        InlineElement::Text(text) => TemplateElement::Text(text.clone()),
        InlineElement::Emphasis(level, line) => match level {
            0 => unreachable!(),
            1 => TemplateElement::Italic(markdown_line_to_template_elements(line)),
            _ => TemplateElement::Bold(markdown_line_to_template_elements(line)),
        },
    }
}
