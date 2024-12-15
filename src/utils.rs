// use std::sync::LazyLock;

// use regex::Regex;

macro_rules! mkregex {
    ($regex_name:ident, $pattern:expr) => {
        static $regex_name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}
pub(crate) use mkregex;
use roxmltree::Node;

pub fn full_node_text<'input>(node: &Node<'_, 'input>) -> &'input str {
    &node.document().input_text()[node.range()]
}
