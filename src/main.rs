use std::io::{self};

use crate::app::App;

mod action;
mod app;
mod components;
mod db_handler;
mod project;
mod state;
mod utils;

// TODO: Build cli interface

fn main() -> Result<(), io::Error> {
    let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
    app.run();
    Ok(())
}
