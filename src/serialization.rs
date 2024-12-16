use std::path::PathBuf;

use crate::{tags::ParsedAO3Tags, utils::vec_as_newlines};
use log::warn;
use rust_xlsxwriter::Worksheet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FullFicInfo {
    pub path_to_file: PathBuf,
    pub title: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub creators: Vec<String>,
    pub tags: ParsedAO3Tags,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FullFicInfoUnwrapped {
    pub path_to_file: PathBuf,
    pub title: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub creators: Vec<String>,

    pub rating: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub archive_warnings: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub categories: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub fandoms: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub relationships: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub characters: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub additional_tags: Vec<String>,
    pub language: Option<String>,
    pub series: Option<String>,
    pub stats: Option<String>,
}

impl From<FullFicInfo> for FullFicInfoUnwrapped {
    fn from(value: FullFicInfo) -> Self {
        FullFicInfoUnwrapped {
            path_to_file: value.path_to_file,
            title: value.title,
            creators: value.creators,

            rating: value.tags.rating,
            archive_warnings: value.tags.archive_warnings,
            categories: value.tags.categories,
            fandoms: value.tags.fandoms,
            relationships: value.tags.relationships,
            characters: value.tags.characters,
            additional_tags: value.tags.additional_tags,
            language: value.tags.language,
            series: value.tags.series,
            stats: value.tags.stats,
        }
    }
}

pub fn write_fic_to_worksheet_row(worksheet: &mut Worksheet, fic_info: &FullFicInfo) -> () {
    let res = worksheet.serialize::<FullFicInfoUnwrapped>(&fic_info.clone().into());
    if let Err(e) = res {
        warn!("could not serialize {:?}\ndue to {}", fic_info, e);
        worksheet.serialize(&format!("{}", e));
    }
}
