use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use bevy_reflect::Reflect;
use itertools::Itertools;
use log::{info, warn};
use rbook::Ebook;
use roxmltree::{Document, Node};
use rust_xlsxwriter::{Format, Workbook, Worksheet};
use walkdir::{DirEntry, WalkDir};

use crate::{
    tags::{AO3Tag, ParsedAO3Tags},
    utils::{full_node_text, vec_as_newlines},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
pub struct FullFicInfo {
    path_to_file: PathBuf,
    title: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    creators: Vec<String>,
    #[serde(flatten)]
    tags: ParsedAO3Tags,
}

pub fn write_fic_to_worksheet_row(worksheet: &mut Worksheet, fic_info: &FullFicInfo) -> () {}

pub fn explore_epub<P: AsRef<Path>>(path: P) -> Result<FullFicInfo>
where
    PathBuf: From<P>,
{
    let epub = rbook::Epub::new(&path)?;
    let reader = epub.reader();
    if let Some(Ok(content)) = reader.fetch_page(0) {
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

        warn!("{:?}", epub.metadata().creators());

        Ok(FullFicInfo {
            path_to_file: path.into(),
            creators: epub
                .metadata()
                .creators()
                .into_iter()
                .map(|elt| elt.value().into())
                .collect(),
            title: epub.metadata().title().map(|elt| elt.value().into()),
            tags: parsed_tags,
        })
    } else {
        bail!("cannot get first page")
    }
}

fn walk_paths_with_epubs<IP: Iterator<Item: AsRef<Path>>>(
    paths: IP,
) -> impl Iterator<Item = DirEntry> {
    paths.flat_map(|path| {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|subpath| {
                subpath.file_type().is_file()
                    && subpath
                        .path()
                        .to_str()
                        .is_some_and(|s| s.ends_with(".epub"))
            })
    })
}

pub fn generate_workbook<P, IP>(workbook_path: P, epub_files_paths: IP) -> Result<()>
where
    P: AsRef<Path>,
    IP: Iterator<Item: AsRef<Path>>,
{
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet: &mut Worksheet = workbook.add_worksheet();

    let format = Format::new().set_bold();

    worksheet.deserialize_headers_with_format::<FullFicInfo>(0, 0, &format)?;

    for fic in walk_paths_with_epubs(epub_files_paths) {
        info!("in cycle");
        match explore_epub(fic.path()) {
            Ok(fic_info) => {
                write_fic_to_worksheet_row(worksheet, &fic_info);
            }
            Err(e) => {
                info!("{}", e);
                continue;
            }
        };
        // info!("{}", serde_yaml::to_string(&parsed_tags)?);
    }

    worksheet.set_column_range_format(0, 100, &Format::new().set_text_wrap())?;
    worksheet.autofit();

    workbook.save(workbook_path)?;
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
