use std::path::PathBuf;

use crate::{tags::ParsedAO3Tags, utils::vec_as_newlines};
use anyhow::bail;
use rust_xlsxwriter::{Format, Worksheet};
use serde::Serialize;
use serde_aux::prelude::serde_introspect;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FicMetaInfo {
    pub path_to_file: PathBuf,
    pub title: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub creators: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FullFicInfo {
    pub meta_info: FicMetaInfo,
    pub tags: ParsedAO3Tags,
}

// pub fn to_flatten_maptree<T>(key_separator: &str, prefix: Option<&str>, src: &T) -> Result<Vec<serde_json::Value>, serde_json::error::Error>
//     where T: serde::Serialize + ?Sized {
//     Ok(serde::ser::SerializeStruct::new(key_separator.into(), prefix.unwrap_or("").into())
//         .disassemble("", "", &serde_json::to_value(src)?))
// }

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct FullFicInfoUnwrapped {
//     pub path_to_file: PathBuf,
//     pub title: Option<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub creators: Vec<String>,

//     pub rating: Option<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub archive_warnings: Vec<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub categories: Vec<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub fandoms: Vec<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub relationships: Vec<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub characters: Vec<String>,
//     #[serde(serialize_with = "vec_as_newlines")]
//     pub additional_tags: Vec<String>,
//     pub language: Option<String>,
//     pub series: Option<String>,
//     pub stats: Option<String>,
// }

// impl From<FullFicInfo> for FullFicInfoUnwrapped {
//     fn from(value: FullFicInfo) -> Self {
//         FullFicInfoUnwrapped {
//             path_to_file: value.path_to_file,
//             title: value.title,
//             creators: value.creators,

//             rating: value.tags.rating,
//             archive_warnings: value.tags.archive_warnings,
//             categories: value.tags.categories,
//             fandoms: value.tags.fandoms,
//             relationships: value.tags.relationships,
//             characters: value.tags.characters,
//             additional_tags: value.tags.additional_tags,
//             language: value.tags.language,
//             series: value.tags.series,
//             stats: value.tags.stats,
//         }
//     }
// }

fn serialize_struct_fields_to_vec_of_string<S: Serialize>(
    strct: &S,
) -> anyhow::Result<Vec<String>> {
    let ser = serde_json::value::to_value(strct)?;
    match ser {
        serde_json::Value::Object(map) => Ok(map.values().map(|elt| elt.to_string()).collect()),
        _ => bail!("bad serde struct: {:?}", ser),
    }
}

pub fn write_headers(worksheet: &mut Worksheet) -> anyhow::Result<()> {
    let fields_names = [
        serde_introspect::<FicMetaInfo>(),
        serde_introspect::<ParsedAO3Tags>(),
    ]
    .concat();
    worksheet.write_row_with_format(0, 0, fields_names, &Format::new().set_bold())?;
    Ok(())
}

pub fn write_fic_to_worksheet_row(
    worksheet: &mut Worksheet,
    row: usize,
    fic_info: &FullFicInfo,
) -> anyhow::Result<()> {
    let res = [
        serialize_struct_fields_to_vec_of_string(&fic_info.meta_info)?,
        serialize_struct_fields_to_vec_of_string(&fic_info.tags)?,
    ]
    .concat();
    worksheet.write_row(row.try_into().unwrap_or(u32::MAX), 0, res)?;
    Ok(())
}
