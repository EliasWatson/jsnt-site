use crate::template::template_page::TemplatePage;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Template {
    name: String,
    pages: HashMap<String, TemplatePage>,
}

impl Template {
    pub fn load(name: &str, path: &PathBuf) -> Self {
        let mut template = Template::default();
        template.name = String::from(name);

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

            let page_name = entry.file_name().to_string_lossy().into_owned();
            let page = TemplatePage::load(page_name.clone(), &entry.path());

            template.pages.insert(page_name, page);
        }

        return template;
    }

    pub fn get_page(&self, tag: &String) -> Option<&TemplatePage> {
        self.pages.get(tag)
    }
}
