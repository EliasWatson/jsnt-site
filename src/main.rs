mod markdown;
mod pages;
mod template;
mod util;

use crate::pages::Pages;
use crate::template::template::Template;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[clap(short, long)]
    out: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let out_path: PathBuf = match args.out {
        Some(path) => path,
        None => args.path.join("out"),
    };

    if !out_path.exists() {
        fs::create_dir(out_path.clone()).unwrap();
    }

    let template = Template::load("test", &args.path.join("template"));
    let pages = Pages::load(&args.path.join("pages"));

    let rendered_pages = pages.render(&template);
    for (page_name, rendered_page) in rendered_pages {
        fs::write(out_path.join(format!("{}.html", page_name)), rendered_page).unwrap();
    }
}
