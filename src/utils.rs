// use std::sync::LazyLock;

// use regex::Regex;

macro_rules! mkregex {
    ($regex_name:ident, $pattern:expr) => {
        static $regex_name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}
use std::path::PathBuf;

pub(crate) use mkregex;
use roxmltree::Node;

pub fn full_node_text<'input>(node: &Node<'_, 'input>) -> &'input str {
    &node.document().input_text()[node.range()]
}

pub fn parse_sequence_of_node_text_children<'a>(
    node: &'a Node<'a, 'a>,
) -> impl Iterator<Item = &'a str> {
    (match node.children().count() {
        0 => vec![*node],
        1 => vec![node.first_child().unwrap()],
        _ => node
            .children()
            .filter(|elt| elt.has_tag_name("a"))
            .collect(),
    })
    .into_iter()
    .map(|s| s.text().unwrap_or("").trim())
}

pub fn vec_as_newlines<S>(v: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&v.join("\n"))
}

pub fn serialize_pathbuf<S>(path: &PathBuf, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.serialize_str(
        &path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or("".into()),
    )
}
