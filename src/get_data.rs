use std::{
    collections::{HashMap, HashSet},
    path::Path,
    usize,
};

use anyhow::{anyhow, bail, Result as AnyResult};
use itertools::Itertools;
use log::{info, warn};
use rbook::{xml::Element, Ebook, Epub};
use roxmltree::Node;
use rust_xlsxwriter::{Format, Workbook, Worksheet};
use walkdir::{DirEntry, WalkDir};

use crate::{
    serialization::{
        write_fic_to_worksheet_row, write_headers, FicMetaInfo, FullFicInfo, ALL_TABLE_COLUMNS,
    },
    tags::{AO3Tag, ParsedAO3Tags},
    utils::full_node_text,
};

pub fn explore_epub<P: AsRef<Path>>(path: P) -> AnyResult<FullFicInfo> {
    let epub = rbook::Epub::new(&path)?;
    let meta_info = extract_fic_meta_info(&path, &epub);
    let tags = extract_fic_tags(&epub);
    Ok(FullFicInfo {
        meta_info,
        tags: tags.map_err(|err| err.to_string()),
    })
    // match extract_fic_tags(&epub) {
    //     Ok(tags) => Ok(FullFicInfo { meta_info, tags }),
    // };
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

pub fn generate_workbook<P, IP>(workbook_path: P, epub_files_paths: IP) -> AnyResult<()>
where
    P: AsRef<Path>,
    IP: Iterator<Item: AsRef<Path>>,
{
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet: &mut Worksheet = workbook.add_worksheet();

    write_headers(worksheet)?;

    for (i, fic) in walk_paths_with_epubs(epub_files_paths).enumerate() {
        info!(
            "exploring epub file `{}`...",
            fic.path().to_str().unwrap_or("")
        );
        match explore_epub(fic.path()) {
            Ok(fic_info) => {
                info!("{:?}", fic_info);
                write_fic_to_worksheet_row(worksheet, i + 1, &fic_info);
            }
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        };
    }

    worksheet.set_column_range_format(
        0,
        (*ALL_TABLE_COLUMNS).len().try_into().unwrap(),
        &Format::new().set_text_wrap(),
    )?;
    worksheet.autofit();

    workbook.save(workbook_path)?;
    Ok(())
}

fn extract_fic_meta_info<P: AsRef<Path>>(path: P, epub: &Epub) -> FicMetaInfo {
    fn extract_vec(v: Vec<&Element>) -> Vec<String> {
        v.into_iter().map(|elt| elt.value().into()).collect()
    }
    fn extract_option(v: Option<&Element>) -> Option<String> {
        v.map(|elt| elt.value().into())
    }

    FicMetaInfo {
        path_to_file: path.as_ref().to_path_buf(),
        creators: extract_vec(epub.metadata().creators()),
        title: extract_option(epub.metadata().title()),
        publisher: extract_vec(epub.metadata().publisher()),
        description: {
            let description = extract_option(epub.metadata().description());
            description.clone().map(|s| {
                html2text::from_read(s.as_bytes(), usize::MAX).unwrap_or_else(|err| {
                    warn!("could not convert description to text: {}", err);
                    description.unwrap()
                })
            })
        },
    }
}

fn extract_fic_tags(epub: &Epub) -> AnyResult<ParsedAO3Tags> {
    let reader = epub.reader();
    let content = match reader.fetch_page(0) {
        Some(Ok(content)) => content,
        None => bail!("could not match first page of document due to lack of such"),
        Some(Err(err)) => bail!(err),
    };
    let content_str = content.as_lossy_str();
    let doc = roxmltree::Document::parse_with_options(
        &content_str,
        roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )
    .map_err(|err| anyhow!("error during parsing: {}", err))?;

    let tags = doc
        .root()
        .descendants()
        .find(|it| it.has_tag_name("dl") && it.attribute("class").unwrap_or("") == "tags")
        .ok_or_else(|| anyhow!("cannot parse document tags"))?;

    let (tags_with_nodes, unknown_tags) = process_dt_dd_elements_to_hash_map(&tags)?;
    if !unknown_tags.is_empty() {
        warn!("Unknown tags encountered: {:?}", unknown_tags)
    }

    Ok(ParsedAO3Tags::from_hash_map_of_ao3tags(&tags_with_nodes))
}

/// here `node` is expected to be a `<dl class="tags">` element
fn process_dt_dd_elements_to_hash_map<'a>(
    node: &'a Node<'a, 'a>,
) -> AnyResult<(HashMap<AO3Tag<&'a str>, Node<'a, 'a>>, HashSet<&'a str>)> {
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
