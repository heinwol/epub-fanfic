use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use anyhow::{anyhow, bail, Ok, Result};
use itertools::Itertools;
use log::warn;
use rbook::Ebook;
use roxmltree::{Document, Node};
use rust_xlsxwriter::Workbook;
use walkdir::WalkDir;

use crate::{
    tags::{AO3Tag, ParsedAO3Tags},
    utils::full_node_text,
};

pub fn explore_epub<P: AsRef<Path>>(path: P) -> Result<()> {
    // Creating an epub instance
    let epub = rbook::Epub::new(&path).unwrap();
    let reader = epub.reader();
    if let Some(std::result::Result::Ok(content)) = reader.fetch_page(0) {
        let content_str = content.as_lossy_str();
        let doc = Document::parse(&content_str)?;

        let tags = doc
            .root()
            .descendants()
            .find(|it| it.has_tag_name("dl") && it.attribute("class").unwrap_or("") == "tags")
            .ok_or_else(|| anyhow!("cannot parse document tags"))?;

        let (tags_with_nodes, unknown_tags) = process_dt_dd_elements_to_hash_map(&tags)?;
        if !unknown_tags.is_empty() {
            warn!("Unknown tags encountered: {:?}", unknown_tags)
        }

        let parsed_tags = ParsedAO3Tags::from_hash_map_of_ao3tags(&tags_with_nodes);

        for category in &parsed_tags.relationships {
            println!("{}", &category);
        }
        println!("{}", parsed_tags.rating.as_ref().unwrap());

        data_to_excel(&parsed_tags)?;
        Ok(())
    } else {
        bail!("cannot get first page")
    }
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

// fn match_tag_text(tag_text: &str) -> &'static str {}

/// here `node` is expected to be a `<dl class="tags">` element
fn process_dt_dd_elements_to_hash_map<'a>(
    node: &'a Node<'a, 'a>,
) -> Result<(HashMap<AO3Tag<&'a str>, Node<'a, 'a>>, HashSet<&'a str>)> {
    assert_eq!(node.attribute("class"), Some("tags"));
    let mut result = HashMap::<AO3Tag<&'a str>, Node<'a, 'a>>::new();
    let mut unknown_tags = HashSet::<&'a str>::new();

    for (dt, dd) in node
        .children()
        // we're only interested in `<dt ...>` and `<dd ...>`
        .filter(|elt| elt.is_element() && ["dt", "dd"].contains(&elt.tag_name().name()))
        .tuples::<(_, _)>()
    {
        if dt.tag_name().name() != "dt" || dd.tag_name().name() != "dd" {
            bail!(
                "Tag pair mismatch during parsing: expected `(<dt ...>, <dd ...>)`, got `({}, {})`",
                full_node_text(&dt),
                full_node_text(&dd)
            )
        }
        // println!("{}", dt.text().unwrap());
        let tag_text = dt
            .text()
            .ok_or_else(|| anyhow!("cannot find text of element `{:?}`", dt))?;
        let tag = AO3Tag::match_str(tag_text);
        if result.contains_key(&tag) {
            match tag {
                AO3Tag::UnknownTag(unknown_tag) => unknown_tags.insert(unknown_tag),
                _ => bail!(
                    "several values found for tag `{}`, namely `{}` and `{}`",
                    tag_text,
                    full_node_text(&result[&tag]),
                    full_node_text(&dd),
                ),
            };
        } else {
            result.insert(tag, dd);
        }
    }

    Ok((result, unknown_tags))
}

fn data_to_excel(data: &ParsedAO3Tags) -> Result<()> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    let format = rust_xlsxwriter::Format::new().set_bold();
    worksheet.deserialize_headers_with_format::<ParsedAO3Tags>(0, 0, &format)?;
    worksheet.serialize(data)?;
    // // Write a string to cell (0, 0) = A1.
    // worksheet.write(0, 0, "Hello")?;

    // // Write a number to cell (1, 0) = A2.
    // worksheet.write(1, 0, 12345)?;

    // Save the file to disk.
    let path = std::path::Path::new("workbook.xlsx");
    workbook.save(path)?;
    Ok(())
}
