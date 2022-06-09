use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Document {
    pub elements: Vec<Element>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Line {
    pub elements: Vec<InlineElement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    Header(u32, Line),
    Paragraph(Vec<Line>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InlineElement {
    Text(String),
    Emphasis(u32, Line),
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        Line {
            elements: vec![InlineElement::Text(s.to_string())],
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for element in self.elements.iter() {
            write!(f, "{}", element)?;
        }

        Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for element in self.elements.iter() {
            write!(f, "{}", element)?;
        }

        Ok(())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Element::Header(level, line) => writeln!(f, "Header[{}] {}", level, line),
            Element::Paragraph(lines) => writeln!(
                f,
                "{}",
                lines
                    .iter()
                    .map(|line| format!("{}", line))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
}

impl fmt::Display for InlineElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InlineElement::Text(s) => write!(f, "{}", s),
            InlineElement::Emphasis(level, line) => write!(f, "Emphasis[{}]({})", level, line),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::markdown::document::{InlineElement, Line};

    #[test]
    fn line_fmt() {
        let assert_line = |strings: Vec<&str>| {
            // There's probably a way better way to do this
            let expected_line = strings
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("");

            let line = Line {
                elements: strings
                    .iter()
                    .map(|s| InlineElement::Text(s.to_string()))
                    .collect(),
            };

            assert_eq!(format!("{}", line), expected_line);
            assert_eq!(line.to_string(), expected_line);
        };

        assert_line(vec![""]);
        assert_line(vec!["Test", " string ", "12", "3"]);
        assert_line(vec!["🍩🍔", "🎸", "💆🎵🌼 🌄"]);
        assert_line(vec!["満", "クぜひ", "参時"]);
    }

    #[test]
    fn inline_element_text_fmt() {
        let assert_text = |s: &str| {
            let text_elem = InlineElement::Text(s.to_string());
            assert_eq!(format!("{}", text_elem), s);
            assert_eq!(text_elem.to_string(), s);
        };

        assert_text("");
        assert_text("Test string 123");
        assert_text("🍩🍔🎸💆🎵🌼 🌄");
        assert_text("満クぜひ参時");
    }
}
