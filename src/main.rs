#![allow(dead_code, unused_variables)]

mod get_data;
mod tags;
mod utils;

use anyhow::Result;
use get_data::main_;

fn main() -> Result<()> {
    main_()?;
    Ok(())
}
