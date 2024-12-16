#![allow(dead_code, unused_variables, unused_must_use)]

mod frontend_iced;
mod get_data;
mod tags;
mod utils;

use anyhow::Result;
// use get_data::generate_workbook;

// use log::info;

// use serde::ser::{Serialize, SerializeStruct, Serializer};

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

// // This is what #[derive(Serialize)] would generate.
// impl Serialize for Person {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Person", 3)?;
//         s.serialize_field("name", &self.name)?;
//         s.serialize_field("age", &self.age)?;
//         s.serialize_field("phones", &self.phones)?;
//         s.end()
//     }
// }

fn main() -> Result<()> {
    let mut clog = colog::default_builder();

    if cfg!(debug_assertions) {
        clog.filter(None, log::LevelFilter::Info);
    } else {
        clog.filter(None, log::LevelFilter::Warn);
    }
    clog.init();

    // frontend_iced::main()?;
    // generate_workbook("./res.xlsx", ["./ignore/"].iter()).unwrap();
    // info!(
    //     "{:?}",
    //     (Person {
    //         age: 12,
    //         name: "Cat".into(),
    //         phones: vec!["+79163333333".into()]
    //     })
    //     .serialize(serde::Serializer)
    // );
    Ok(())
}
