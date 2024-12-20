use std::{path::PathBuf, sync::LazyLock};

use crate::{
    tags::ParsedAO3Tags,
    utils::{pub_static_with_lock, static_with_lock, vec_as_newlines},
};
use anyhow::bail;
use log::warn;
use rust_xlsxwriter::{Color, Format, Worksheet};
use serde::Serialize;
use serde_aux::prelude::serde_introspect;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct FicMetaInfo {
    pub path_to_file: PathBuf,
    pub title: Option<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub creators: Vec<String>,
    #[serde(serialize_with = "vec_as_newlines")]
    pub publisher: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FullFicInfo {
    pub meta_info: FicMetaInfo,
    pub tags: Result<ParsedAO3Tags, String>,
}

static_with_lock!(
    FICMETAINFO_FIELD_NAMES,
    Vec<&str>,
    serde_introspect::<FicMetaInfo>().into()
);
static_with_lock!(
    PARSEDAO3TAGS_FIELD_NAMES,
    Vec<&str>,
    serde_introspect::<ParsedAO3Tags>().into()
);
pub_static_with_lock!(
    ALL_TABLE_COLUMNS,
    Vec<&str>,
    [&FICMETAINFO_FIELD_NAMES[..], &PARSEDAO3TAGS_FIELD_NAMES[..]].concat()
);

fn serialize_struct_fields_to_vec_of_string<S: Serialize>(
    strct: &S,
    field_names: &'static [&str],
) -> anyhow::Result<Vec<String>> {
    let ser = serde_json::value::to_value(strct)?;
    match ser {
        serde_json::Value::Object(map) => Ok(field_names
            .iter()
            .map(|&field_name| {
                let field_value = map[field_name].to_owned();
                let parsed_value = serde_json::from_value::<Option<String>>(field_value.clone());
                parsed_value
                    .unwrap_or_else(|err| {
                        warn!("cannot convert serialized data to string:\n\t{}", err);
                        Some("".into())
                    })
                    .unwrap_or("".into())
            })
            .collect()),
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
) -> () {
    let row = row.try_into().unwrap_or(u32::MAX);
    let mut perform_operation = || -> anyhow::Result<()> {
        let vec_of_meta_info_fields = serialize_struct_fields_to_vec_of_string(
            &fic_info.meta_info,
            &FICMETAINFO_FIELD_NAMES,
        )?;
        let n_info_fields_cols: u16 = vec_of_meta_info_fields.len().try_into().unwrap();
        worksheet.write_row(row, 0, vec_of_meta_info_fields)?;

        match &fic_info.tags {
            Ok(tags) => {
                let vec_of_tags_fields =
                    serialize_struct_fields_to_vec_of_string(&tags, &PARSEDAO3TAGS_FIELD_NAMES)?;
                worksheet.write_row(row, n_info_fields_cols, vec_of_tags_fields)?;
            }
            Err(err) => {
                worksheet.write_string_with_format(
                    row,
                    n_info_fields_cols,
                    err,
                    &Format::new().set_font_color(Color::Red),
                )?;
            }
        }

        Ok(())
    };

    perform_operation().inspect_err(|err| {
        worksheet.write_string_with_format(
            row,
            0,
            err.backtrace().to_string(),
            &Format::new().set_font_color(Color::Red),
        );
    });
}
