use crate::markdown::builder::ParagraphBuilder;
use crate::markdown::document::{Document, Element, InlineElement, Line};
use crate::util::regex_split::split_by_regex;
use fancy_regex::Regex;
use lazy_static::lazy_static;

macro_rules! return_if_some {
    ($a:expr) => {
        match $a {
            Some(x) => return x,
            None => {}
        }
    };
}

macro_rules! return_option_if_some {
    ($a:expr) => {
        match $a {
            Some(x) => return Some(x),
            None => {}
        }
    };
}

pub fn parse(raw_text: &str) -> Document {
    let mut elements = vec![];
    let mut paragraph_builder = ParagraphBuilder::default();

    for line in raw_text.lines() {
        match parse_raw_line(line) {
            Some(element) => match element {
                Element::Paragraph(lines) => {
                    paragraph_builder.add_lines(lines);
                }
                _ => {
                    paragraph_builder.finish(&mut elements);
                    elements.push(element);
                }
            },
            None => {
                paragraph_builder.finish(&mut elements);
            }
        };
    }

    paragraph_builder.finish(&mut elements);

    return Document { elements };
}

fn parse_raw_line(line: &str) -> Option<Element> {
    if line.trim().len() == 0 {
        return None;
    }

    return_option_if_some!(parse_header(line));

    return Some(Element::Paragraph(vec![parse_line(line)]));
}

fn parse_header(line: &str) -> Option<Element> {
    lazy_static! {
        static ref HEADER_PATTERN: Regex = Regex::new(r"^(#+)\s+(.*)$").unwrap();
    }

    let caps = match HEADER_PATTERN.captures(line) {
        Ok(c) => c,
        Err(_) => return None,
    }?;

    return Some(Element::Header(
        caps.get(1).unwrap().as_str().len() as u32,
        parse_line(caps.get(2).unwrap().as_str()),
    ));
}

fn parse_line(line: &str) -> Line {
    if line.trim().len() == 0 {
        return Line::from_str("");
    }

    return_if_some!(parse_emphasis(line));

    return Line::from_str(line);
}

fn parse_emphasis(line: &str) -> Option<Line> {
    lazy_static! {
        static ref EMPHASIS_PATTERN: Regex = Regex::new(r"((?:\*|_){1,2})(.+?)\1").unwrap();
    }

    match EMPHASIS_PATTERN.is_match(line) {
        Ok(b) => {
            if !b {
                return None;
            }
        }
        Err(_) => return None,
    }

    let elements = split_by_regex(
        line,
        &*EMPHASIS_PATTERN,
        |captures| {
            InlineElement::Emphasis(
                captures.get(1).unwrap().as_str().len() as u32,
                parse_line(captures.get(2).unwrap().as_str()),
            )
        },
        |text| InlineElement::Text(text.to_string()), // TODO
    );

    return Some(Line { elements });
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
                    Element::Paragraph(vec![Line::from_str("And more text under the header.")]),
                    Element::Paragraph(vec![Line::from_str(
                        "This is a separate paragraph under the same heading."
                    )]),
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
