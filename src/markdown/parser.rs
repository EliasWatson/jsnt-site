use crate::markdown::document::{Document, Element, InlineElement, Line};
use lazy_static::lazy_static;
use regex::Regex;
use std::mem;

macro_rules! return_if_some {
    ($a:expr) => {
        match $a {
            Some(x) => return Some(x),
            None => {}
        }
    };
}

pub fn parse(raw_text: &str) -> Document {
    let mut elements = vec![];
    let mut current_paragraph: Vec<Line> = vec![];

    for line in raw_text.lines() {
        match parse_raw_line(line) {
            Some(element) => match element {
                Element::Paragraph(paragraph_elements) => {
                    current_paragraph.extend(paragraph_elements);
                }
                _ => {
                    if current_paragraph.len() > 0 {
                        elements.push(Element::Paragraph(mem::replace(
                            &mut current_paragraph,
                            vec![],
                        )));
                    }
                    elements.push(element);
                }
            },
            None => {}
        };
    }

    if current_paragraph.len() > 0 {
        elements.push(Element::Paragraph(current_paragraph));
    }

    return Document { elements };
}

fn parse_raw_line(line: &str) -> Option<Element> {
    if line.trim().len() == 0 {
        return None;
    }

    return_if_some!(parse_header(line));

    return Some(Element::Paragraph(vec![parse_line(line)]));
}

fn parse_header(line: &str) -> Option<Element> {
    lazy_static! {
        static ref HEADER_PATTERN: Regex = Regex::new(r"^(#+)\s+(.*)$").unwrap();
    }

    let caps = HEADER_PATTERN.captures(line)?;

    return Some(Element::Header(
        caps.get(1).unwrap().as_str().len() as u32,
        parse_line(caps.get(2).unwrap().as_str()),
    ));
}

fn parse_line(line: &str) -> Line {
    // TODO: Parse links, bold, italic, etc
    Line::from_str(line)
}

#[cfg(test)]
mod test {
    use crate::markdown::document::{Document, Element, InlineElement, Line};
    use crate::markdown::parser::{parse, parse_header, parse_line};
    use std::fs;

    #[test]
    fn test_parse_with_headings_paragraphs() {
        let raw_text = fs::read_to_string("test_data/markdown/headings_paragraphs.md").unwrap();
        let parsed_doc = parse(&raw_text);

        assert_eq!(
            parsed_doc,
            Document {
                elements: vec![
                    Element::Header(1, Line::from_str("Test of my markdown parser")),
                    Element::Paragraph(vec![
                        Line::from_str("This is a test file for my markdown parser."),
                        Line::from_str("This file only contains headings and paragraphs.")
                    ]),
                    Element::Header(1, Line::from_str("This is a second header")),
                    Element::Paragraph(vec![Line::from_str("And more text under the header."),]),
                    Element::Header(2, Line::from_str("Here is a sub-header")),
                    Element::Paragraph(vec![
                        Line::from_str("Random text here."),
                        Line::from_str("A second line of random text."),
                        Line::from_str("A third line of random text."),
                    ]),
                ]
            }
        );
    }

    #[test]
    fn test_parse_header() {
        let gen_header_prefix = |level: u32| String::from("#").repeat(level as usize);
        assert_eq!(gen_header_prefix(1), "#");
        assert_eq!(gen_header_prefix(2), "##");
        assert_eq!(gen_header_prefix(3), "###");

        let assert_header = |s: &str| {
            for level in 1..10 {
                let raw_text = format!("{} {}", gen_header_prefix(level), s);
                assert_eq!(
                    parse_header(&raw_text),
                    Some(Element::Header(level, Line::from_str(s)))
                );
            }
        };

        assert_eq!(parse_header(""), None);
        assert_eq!(parse_header("A test sentence"), None);

        assert_header("");
        assert_header("A test sentence");
    }

    #[test]
    fn test_parse_line() {
        let assert_line = |s: &str| {
            assert_eq!(parse_line(s), Line::from_str(s));
        };

        assert_line("");
        assert_line("A test sentence");
    }
}
