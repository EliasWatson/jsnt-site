use crate::template::template_errors::TemplateError;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TemplateString {
    sections: Vec<TemplateStringSection>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TemplateStringSection {
    Text(String),
    Variable(String),
    DefaultVariable(String, String),
}

impl TemplateString {
    pub fn parse_string(s: &str) -> Self {
        lazy_static! {
            static ref VARIABLE_PATTERN: Regex = Regex::new(r"\{([\w\d\-_]+)\}").unwrap();
        }

        let mut sections = vec![];

        let mut prev_end_index = 0;
        for cap in VARIABLE_PATTERN.captures_iter(s) {
            let entire_match = cap.get(0).unwrap();

            if entire_match.start() > prev_end_index {
                sections.push(TemplateStringSection::Text(String::from(
                    &s[prev_end_index..entire_match.start()],
                )));
            }

            prev_end_index = entire_match.end();

            sections.push(TemplateStringSection::Variable(String::from(
                cap.get(1).unwrap().as_str(),
            )));
        }

        // This is `s.len()` not `s.len() - 1` because end_index is always one past the last variable
        if prev_end_index < s.len() {
            sections.push(TemplateStringSection::Text(String::from(
                &s[prev_end_index..],
            )));
        }

        return TemplateString { sections };
    }

    pub fn set(&mut self, name: &str, value: &str) {
        for section in self.sections.iter_mut() {
            match section {
                TemplateStringSection::Variable(var_name)
                | TemplateStringSection::DefaultVariable(var_name, _)
                    if var_name == name => {}
                _ => continue,
            };

            *section = TemplateStringSection::Text(String::from(value));
        }
    }

    pub fn render(&self) -> Result<String, TemplateError> {
        let mut strings: Vec<String> = vec![];
        let mut missing_variables: HashSet<String> = HashSet::new();

        for section in &self.sections {
            match section {
                TemplateStringSection::Text(text) => {
                    strings.push(text.clone());
                }
                TemplateStringSection::DefaultVariable(_, default_text) => {
                    strings.push(default_text.clone());
                }
                TemplateStringSection::Variable(name) => {
                    missing_variables.insert(name.clone());
                }
            };
        }

        if missing_variables.len() > 0 {
            return Err(TemplateError::MissingVariables(missing_variables));
        }

        return Ok(strings.join(""));
    }
}

#[cfg(test)]
mod test {
    use crate::template::template_errors::TemplateError;
    use crate::template::template_string::{TemplateString, TemplateStringSection};
    use std::collections::HashSet;

    macro_rules! text {
        ($a:expr) => {
            TemplateStringSection::Text($a.to_string())
        };
    }

    macro_rules! var {
        ($a:expr) => {
            TemplateStringSection::Variable($a.to_string())
        };
    }

    const TEST_STR_1: &str =
        "<h{level}>{content} + some other text {extra}\\{nomatch\\}</h{level}>";
    const TEST_STR_2: &str = "{prefix}{suffix}";
    const TEST_STR_3: &str = "No variables in this template string";

    fn get_test_str_1_sections() -> Vec<TemplateStringSection> {
        vec![
            text!("<h"),
            var!("level"),
            text!(">"),
            var!("content"),
            text!(" + some other text "),
            var!("extra"),
            text!("\\{nomatch\\}</h"),
            var!("level"),
            text!(">"),
        ]
    }

    fn get_test_str_2_sections() -> Vec<TemplateStringSection> {
        vec![var!("prefix"), var!("suffix")]
    }

    fn get_test_str_3_sections() -> Vec<TemplateStringSection> {
        vec![text!(TEST_STR_3)]
    }

    #[test]
    fn test_parse_string() {
        let template_1 = TemplateString::parse_string(TEST_STR_1);
        assert_eq!(template_1.sections, get_test_str_1_sections());

        let template_2 = TemplateString::parse_string(TEST_STR_2);
        assert_eq!(template_2.sections, get_test_str_2_sections());

        let template_3 = TemplateString::parse_string(TEST_STR_3);
        assert_eq!(template_3.sections, get_test_str_3_sections());
    }

    #[test]
    fn test_set() {
        // Test String 1
        let mut template_1 = TemplateString::parse_string(TEST_STR_1);

        let mut template_1_sections = get_test_str_1_sections();
        assert_eq!(template_1.sections, template_1_sections);

        template_1.set("content", "some value");
        assert_ne!(template_1.sections, template_1_sections);
        template_1_sections[3] = text!("some value");
        assert_eq!(template_1.sections, template_1_sections);

        template_1.set("missing", "this variable does not exist");
        assert_eq!(template_1.sections, template_1_sections);

        template_1.set("level", "42");
        assert_ne!(template_1.sections, template_1_sections);
        template_1_sections[1] = text!("42");
        assert_ne!(template_1.sections, template_1_sections);
        template_1_sections[7] = text!("42");
        assert_eq!(template_1.sections, template_1_sections);

        template_1.set("extra", "sample");
        assert_ne!(template_1.sections, template_1_sections);
        template_1_sections[5] = text!("sample");
        assert_eq!(template_1.sections, template_1_sections);

        // Test String 2
        let mut template_2 = TemplateString::parse_string(TEST_STR_2);

        let mut template_2_sections = get_test_str_2_sections();
        assert_eq!(template_2.sections, template_2_sections);

        template_2.set("prefix", "first");
        assert_ne!(template_2.sections, template_2_sections);
        template_2_sections[0] = text!("first");
        assert_eq!(template_2.sections, template_2_sections);

        template_2.set("suffix", "last");
        assert_ne!(template_2.sections, template_2_sections);
        template_2_sections[1] = text!("last");
        assert_eq!(template_2.sections, template_2_sections);

        // Test String 2
        let mut template_3 = TemplateString::parse_string(TEST_STR_3);

        let template_3_sections = get_test_str_3_sections();
        assert_eq!(template_3.sections, template_3_sections);

        template_3.set("missing", "this variable does not exist");
        assert_eq!(template_3.sections, template_3_sections);
    }

    #[test]
    fn test_render() {
        let mut template_1 = TemplateString::parse_string(TEST_STR_1);
        template_1.set("level", "2");
        template_1.set("content", "variable text");
        template_1.set("extra", "for testing");
        assert_eq!(
            template_1.render().unwrap(),
            "<h2>variable text + some other text for testing\\{nomatch\\}</h2>".to_string()
        );

        let mut template_2 = TemplateString::parse_string(TEST_STR_2);
        template_2.set("suffix", ")");
        template_2.set("prefix", ":");
        assert_eq!(template_2.render().unwrap(), ":)".to_string());

        let template_3 = TemplateString::parse_string(TEST_STR_3);
        assert_eq!(template_3.render().unwrap(), TEST_STR_3.to_string());
    }

    #[test]
    fn test_render_errors() {
        let mut template_1 = TemplateString::parse_string(TEST_STR_1);
        assert_eq!(
            template_1.render(),
            Err(TemplateError::MissingVariables(HashSet::from([
                "level".to_string(),
                "content".to_string(),
                "extra".to_string()
            ])))
        );

        template_1.set("level", "2");
        assert_eq!(
            template_1.render(),
            Err(TemplateError::MissingVariables(HashSet::from([
                "content".to_string(),
                "extra".to_string()
            ])))
        );

        template_1.set("content", "variable text");
        assert_eq!(
            template_1.render(),
            Err(TemplateError::MissingVariables(HashSet::from([
                "extra".to_string()
            ])))
        );
    }

    #[test]
    fn test_default_variables() {
        let mut template = TemplateString {
            sections: vec![
                TemplateStringSection::Text("This is a test of ".to_string()),
                TemplateStringSection::DefaultVariable(
                    "name".to_string(),
                    "default variables".to_string(),
                ),
                TemplateStringSection::Variable("ending".to_string()),
            ],
        };
        assert_eq!(
            template.render(),
            Err(TemplateError::MissingVariables(HashSet::from([
                "ending".to_string()
            ])))
        );

        template.set("ending", "!");
        assert_eq!(
            template.render().unwrap(),
            "This is a test of default variables!".to_string()
        );

        template.set("name", "nothing");
        assert_eq!(
            template.render().unwrap(),
            "This is a test of nothing!".to_string()
        );
    }
}
