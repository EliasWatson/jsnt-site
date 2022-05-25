use crate::template::template_page::TemplatePage;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Template {
    name: String,
    pages: HashMap<String, TemplatePage>,
}
