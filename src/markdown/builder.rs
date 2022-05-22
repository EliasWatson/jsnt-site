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
