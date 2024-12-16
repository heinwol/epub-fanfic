#![allow(dead_code, unused_variables, unused_must_use)]
#![windows_subsystem = "windows"]

mod frontend_iced;
mod get_data;
mod serialization;
mod tags;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    let mut clog = colog::default_builder();

    if cfg!(debug_assertions) {
        clog.filter(None, log::LevelFilter::Info);
    } else {
        clog.filter(None, log::LevelFilter::Warn);
    }
    clog.init();

    frontend_iced::main()?;
    // get_data::generate_workbook("./res.xlsx", ["./ignore/"].iter()).unwrap();
    Ok(())
}
