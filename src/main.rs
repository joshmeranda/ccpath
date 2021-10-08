#[macro_use]
extern crate clap;

use crate::convert_path::{convert_full, Convention};
use crate::error::PathConvertError;
use clap::Arg;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::process::exit;

mod convert_path;
mod error;

// todo: convert full path (relative and absolute)
// todo: convert just the basename
// todo: handle overwriting a file
fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("recursive").short("r").long("recursive"))
        .arg(Arg::with_name(("dry-run")).long("dry-run"))
        .arg(
            Arg::with_name("basename")
                .conflicts_with("full-path")
                .short("b")
                .long("basename"),
        )
        .arg(
            Arg::with_name("full-path")
                .conflicts_with("full-path")
                .short("F")
                .long("full-path"),
        )
        .arg(Arg::with_name("from").short("f").long("--from"))
        .arg(
            Arg::with_name("into")
                .short("-t")
                .long("--into")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("paths")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let from_convention = if matches.is_present("from") {
        match Convention::try_from(matches.value_of("from").unwrap()) {
            Ok(c) => Some(c),
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(1);
            }
        }
    } else {
        None
    };

    let to_convention = match Convention::try_from(matches.value_of("into").unwrap()) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    };

    let paths: Vec<&Path> = matches
        .values_of("paths")
        .unwrap()
        .map(|v| Path::new(v))
        .collect();
}
