#[macro_use]
extern crate clap;

mod convert_path;
mod error;

use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgGroup};

use crate::convert_path::Convention;

// todo: handle overwriting a file
fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("recursive")
                .help("recurse into a directory")
                .short("r")
                .long("recursive"),
        )
        .arg(
            Arg::with_name("dry-run")
                .help("show the operations that would be performed without doing them")
                .long("dry-run"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("print a message for every converted path")
                .short("v")
                .long("verbose"),
        )
        .arg(
            Arg::with_name("basename")
                .help("only convert the basename of each given path")
                .short("b")
                .long("basename"),
        )
        .arg(
            Arg::with_name("full-path")
                .help("convert all components of the path")
                .short("F")
                .long("full-path"),
        )
        .arg(
            Arg::with_name("prefix")
                .help("exclude a path prefix when converting a '--full-path' is specified, otherwise ignored'")
                .short("P")
                .long("prefix")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("from")
                .help("set the current naming convention if it is known, this may improve teh case conversion accuracy")
                .short("f")
                .long("from")
                .value_name("CONVENTION")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("into")
                .help("set that target naming convention")
                .value_name("CONVENTION")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("paths")
                .help("the paths to convert")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .group(ArgGroup::with_name("mode").args(&["basename", "full-path"]))
        .after_help("ccpath supports several naming conventions:\n  \
                    title  Title Case\n  \
                    flat   flatcase\n  \
                    FLAT   UPPERFLATCASE\n  \
                    camel  camelCase\n  \
                    CAMEL  CamelCase\n  \
                    snake  snake_case\n  \
                    SNAKE  SNAKE_CASE\n  \
                    kebab  kebab-case\n"
        )
        .get_matches();

    let is_verbose = matches.is_present("verbose");
    let is_dry_run = matches.is_present("dry-run");

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

    for path in paths {
        if !path.exists() {
            eprintln!("Error: no such file or directory '{}'", path.display());
            exit(2);
        }

        // todo: move this check outside loop
        //       store method or closure reference outside loop or run separate loops
        let result = if matches.is_present("full-path") {
            if matches.is_present("prefix") {
                convert_path::convert_full_except_prefix(path, matches.value_of("prefix").unwrap(), from_convention, to_convention)
            }else {
                convert_path::convert_full(path, from_convention, to_convention)
            }
        } else {
            convert_path::convert_basename(path, from_convention, to_convention)
        };

        let new_path = match result {
            Ok(p) => p,
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(3);
            }
        };

        if !is_dry_run {
            let parent = new_path.parent();
            if parent.is_some() && ! parent.unwrap().exists() {

                if let Err(err) = fs::create_dir_all(parent.unwrap()) {
                    eprintln!("Error: {}", err);
                    exit(4);
                }

                println!("=== 100 '{}' : {} ===", parent.unwrap().display(), parent.unwrap().exists());
            }

            if let Err(err) = fs::rename(path, new_path.to_path_buf()) {
                eprintln!("Error: {}", err);
            }
        }

        if is_verbose || is_dry_run {
            println!(
                "'{}' -> '{}'",
                path.to_str().unwrap(),
                new_path.to_str().unwrap()
            );
        }
    }
}
