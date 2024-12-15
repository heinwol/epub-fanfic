use std::{borrow::Borrow, collections::HashMap, sync::LazyLock};

use regex::Regex;
use roxmltree::Node;

use crate::utils::mkregex;

fn vec_as_newlines<S>(v: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&v.join("\n"))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedAO3Tags {
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

impl ParsedAO3Tags {
    pub fn from_hash_map_of_ao3tags<'a>(hash_map: &HashMap<AO3Tag<&'a str>, Node<'a, 'a>>) -> Self {
        ParsedAO3Tags {
            rating: get_tag_opt(&AO3Tag::Rating, hash_map),
            archive_warnings: get_tag_vec(&AO3Tag::ArchiveWarnings, hash_map),
            categories: get_tag_vec(&AO3Tag::Categories, hash_map),
            fandoms: get_tag_vec(&AO3Tag::Fandoms, hash_map),
            relationships: get_tag_vec(&AO3Tag::Relationships, hash_map),
            characters: get_tag_vec(&AO3Tag::Characters, hash_map),
            additional_tags: get_tag_vec(&AO3Tag::AdditionalTags, hash_map),
            language: get_tag_opt(&AO3Tag::Language, hash_map),
            series: get_tag_opt(&AO3Tag::Series, hash_map),
            stats: get_tag_opt(&AO3Tag::Stats, hash_map),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AO3Tag<S: Borrow<str>> {
    Rating,
    ArchiveWarnings,
    Categories,
    Fandoms,
    Relationships,
    Characters,
    AdditionalTags,
    Language,
    Series,
    Stats,
    UnknownTag(S),
}

impl<S: Borrow<str>> AO3Tag<S> {
    pub fn match_str(s: S) -> Self {
        if RE_RATING.find(s.borrow()).is_some() {
            Self::Rating
        } else if RE_ARCHIVE_WARNINGS.find(s.borrow()).is_some() {
            Self::ArchiveWarnings
        } else if RE_CATEGORIES.find(s.borrow()).is_some() {
            Self::Categories
        } else if RE_FANDOMS.find(s.borrow()).is_some() {
            Self::Fandoms
        } else if RE_RELATIONSHIPS.find(s.borrow()).is_some() {
            Self::Relationships
        } else if RE_CHARACTERS.find(s.borrow()).is_some() {
            Self::Characters
        } else if RE_ADDITIONAL_TAGS.find(s.borrow()).is_some() {
            Self::AdditionalTags
        } else if RE_LANGUAGE.find(s.borrow()).is_some() {
            Self::Language
        } else if RE_SERIES.find(s.borrow()).is_some() {
            Self::Series
        } else if RE_STATS.find(s.borrow()).is_some() {
            Self::Stats
        } else {
            Self::UnknownTag(s)
        }
    }

    // pub fn parse_tag_contents<'a>(&self, dd_tag: &'a Node<'a, 'a>) ->
}

mkregex!(RE_RATING, r"(?i).*rating.*");
mkregex!(RE_ARCHIVE_WARNINGS, r"(?i).*archiv.*warn.*");
mkregex!(RE_CATEGORIES, r"(?i).*categor.*");
mkregex!(RE_FANDOMS, r"(?i).*fandom.*");
mkregex!(RE_RELATIONSHIPS, r"(?i).*relationship.*");
mkregex!(RE_CHARACTERS, r"(?i).*character.*");
mkregex!(RE_ADDITIONAL_TAGS, r"(?i).*addit.*tag.*");
mkregex!(RE_LANGUAGE, r"(?i).*lang.*");
mkregex!(RE_SERIES, r"(?i).*series.*");
mkregex!(RE_STATS, r"(?i).*stat.*");

fn parse_sequence_of_dd_children<'a>(node: &'a Node<'a, 'a>) -> Vec<&'a str> {
    match node.children().count() {
        0 => vec![node.text().unwrap_or("")],
        1 => vec![node.first_child().unwrap().text().unwrap_or("")],
        _ => node
            .children()
            .filter(|elt| elt.has_tag_name("a"))
            .map(|elt| elt.text().unwrap_or(""))
            .collect(),
    }
}

fn get_tag_opt<'a>(
    tag: &AO3Tag<&'a str>,
    hash_map: &'a HashMap<AO3Tag<&'a str>, Node<'a, 'a>>,
) -> Option<String> {
    if !hash_map.contains_key(tag) {
        None
    } else {
        parse_sequence_of_dd_children(&hash_map[tag])
            .first()
            .map(|&s| s.into())
    }
}

fn get_tag_vec<'a>(
    tag: &AO3Tag<&'a str>,
    hash_map: &'a HashMap<AO3Tag<&'a str>, Node<'a, 'a>>,
) -> Vec<String> {
    if !hash_map.contains_key(tag) {
        vec![]
    } else {
        parse_sequence_of_dd_children(&hash_map[tag])
            .into_iter()
            .map(|s| s.into())
            .collect()
    }
}
