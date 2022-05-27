use crate::markdown::document::{Element, Line};
use std::mem;

#[derive(Debug, Default)]
pub struct ParagraphBuilder {
    lines: Vec<Line>,
}

impl ParagraphBuilder {
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn add_lines(&mut self, lines: Vec<Line>) {
        self.lines.extend(lines);
    }

    pub fn add_str(&mut self, s: &str) {
        self.add_line(Line::from_str(s));
    }

    pub fn finish(&mut self, out_vec: &mut Vec<Element>) {
        if self.lines.len() > 0 {
            out_vec.push(Element::Paragraph(mem::replace(&mut self.lines, vec![])));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::markdown::builder::ParagraphBuilder;
    use crate::markdown::document::{Element, Line};

    #[test]
    fn test_paragraph_builder() {
        let mut out_vec = vec![];

        let mut builder = ParagraphBuilder::default();
        builder.add_line(Line::from_str("A test line"));
        builder.finish(&mut out_vec);
        assert_eq!(
            out_vec,
            vec![Element::Paragraph(vec![Line::from_str("A test line")])]
        );
        out_vec.clear();

        let mut builder = ParagraphBuilder::default();
        builder.add_line(Line::from_str("line one"));
        builder.add_line(Line::from_str("second line"));
        builder.finish(&mut out_vec);
        assert_eq!(
            out_vec,
            vec![Element::Paragraph(vec![
                Line::from_str("line one"),
                Line::from_str("second line")
            ])]
        );
        out_vec.clear();

        let mut builder = ParagraphBuilder::default();
        builder.add_lines(vec![
            Line::from_str("line one"),
            Line::from_str("second line"),
        ]);
        builder.finish(&mut out_vec);
        assert_eq!(
            out_vec,
            vec![Element::Paragraph(vec![
                Line::from_str("line one"),
                Line::from_str("second line")
            ])]
        );
        out_vec.clear();

        let mut builder = ParagraphBuilder::default();
        builder.add_str("line one");
        builder.add_str("second line");
        builder.finish(&mut out_vec);
        assert_eq!(
            out_vec,
            vec![Element::Paragraph(vec![
                Line::from_str("line one"),
                Line::from_str("second line")
            ])]
        );
        out_vec.clear();
    }
}
