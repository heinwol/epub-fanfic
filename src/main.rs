#![allow(dead_code, unused_variables)]

mod get_data;
mod tags;
mod utils;

use anyhow::Result;
use get_data::main_;

fn main() -> Result<()> {
    let mut clog = colog::default_builder();

    if cfg!(debug_assertions) {
        clog.filter(None, log::LevelFilter::Info);
    } else {
        clog.filter(None, log::LevelFilter::Warn);
    }
    clog.init();
    main_()?;
    Ok(())
}
