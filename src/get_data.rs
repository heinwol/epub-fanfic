use std::path::Path;

use anyhow::{anyhow, bail, Result};
// use rbook::read::ContentType;
use rbook::Ebook;
use roxmltree::{Document, Node};
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize)]
struct ParsedTags<S: AsRef<str>> {
    rating: Option<S>,
    archive_warnings: Vec<S>,
    categories: Vec<S>,
    fandoms: Vec<S>,
    relationships: Vec<S>,
    character: Option<S>,
    additional_tags: Vec<S>,
    language: Option<S>,
    series: Option<S>,
    stats: Option<S>,
}

pub fn explore_epub<P: AsRef<Path>>(path: P) -> Result<()> {
    // Creating an epub instance
    let epub = rbook::Epub::new(&path).unwrap();
    let reader = epub.reader();
    // let _ = epub.
    if let Some(Ok(content)) = reader.fetch_page(0) {
        let content_str = content.as_lossy_str();
        let doc = Document::parse(&content_str)?;

        let tags = doc
            .root()
            .descendants()
            .find(|it| it.has_tag_name("dl") && it.attribute("class").unwrap_or("") == "tags")
            .ok_or(anyhow!("cannot parse document tags"))?;
        let first_dt = tags
            .descendants()
            .find(|it| it.has_tag_name("dt"))
            .ok_or(anyhow!("cannot find dt element"))?;

        for it in tags.children() {
            println!("{:?}", it.children().collect::<Vec<_>>());
        }
        Ok(())
    } else {
        bail!("cannot get first page")
    }
    // let manifest = epub.manifest();
    // println!("{:?}", manifest);

    // for element in content.elements() {
    //     println!("title:{}, href:{}", element.name(), element.value());
    // }
    // let elements = epub.metadata().elements();
    // // let elt = elements[0];
    // for elt in elements {
    //     println!("{:?}", elt);
    // }
}

pub fn main_() -> Result<()> {
    for fic in WalkDir::new("ignore")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|subpath| {
            subpath.file_type().is_file()
                && subpath
                    .path()
                    .to_str()
                    .is_some_and(|s| s.ends_with(".epub"))
        })
    {
        explore_epub(fic.path())?;
        println!("{}\n", "-".repeat(100));
    }
    Ok(())
}

/// here `node` is expected to be a "dt" tag
fn analyze_dt_dd_elements(node: &Node) -> () {}
