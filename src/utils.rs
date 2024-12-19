use roxmltree::Node;

macro_rules! mkregex {
    ($regex_name:ident, $pattern:expr) => {
        static $regex_name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}

macro_rules! static_with_lock {
    ($static_name:ident, $type_:ty, $pattern:expr) => {
        static $static_name: LazyLock<$type_> = LazyLock::new(|| $pattern);
    };
}
macro_rules! pub_static_with_lock {
    ($static_name:ident, $type_:ty, $pattern:expr) => {
        pub static $static_name: LazyLock<$type_> = LazyLock::new(|| $pattern);
    };
}

pub(crate) use mkregex;
pub(crate) use pub_static_with_lock;
pub(crate) use static_with_lock;

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

// pub fn serialize_pathbuf<S>(path: &PathBuf, ser: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     ser.serialize_str(
//         &path
//             .clone()
//             .into_os_string()
//             .into_string()
//             .unwrap_or("".into()),
//     )
// }
