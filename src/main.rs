#![allow(dead_code, unused_variables, unused_must_use)]

mod frontend;
mod get_data;
mod tags;
mod utils;

use anyhow::Result;
use get_data::generate_workbook;
// use rfd::FileDialog;

fn main() -> Result<()> {
    let mut clog = colog::default_builder();

    if cfg!(debug_assertions) {
        clog.filter(None, log::LevelFilter::Info);
    } else {
        clog.filter(None, log::LevelFilter::Warn);
    }
    clog.init();

    // let files = FileDialog::new()
    //     .add_filter("epub", &["epub"])
    //     .set_directory(current_dir().unwrap_or(".".into()))
    //     .pick_files();
    // println!("{:?}", files);

    frontend::main().unwrap();

    generate_workbook()?;
    Ok(())
}
