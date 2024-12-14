use std::path::Path;

use rbook::read::ContentType;
use rbook::Ebook;

pub fn explore_epub<P: AsRef<Path>>(path: P) -> () {
    // Creating an epub instance
    let epub = rbook::Epub::new(path).unwrap();
    let elements = epub.metadata().elements();
    // let elt = elements[0];
    for elt in elements {
        println!("{:?}", elt)
    }
}

pub fn main_() -> () {
    for fic in ["Worlds_Contract", "Fall_From_Grace"] {
        explore_epub(format!("ignore/{fic}.epub"));
        println!("{}", "-".repeat(100))
    }
}
