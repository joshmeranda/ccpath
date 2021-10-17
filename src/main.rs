#[macro_use]
extern crate clap;

use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgGroup, ArgMatches};
use walkdir::WalkDir;

use convert_path::{self, Convention};
use convert_path::error::PathConvertError;

fn get_matches<'a>() -> ArgMatches<'a> {
    app_from_crate!()
        .arg(
            Arg::with_name("recursive")
                .help("recurse into a directory, works in the same way as using '--prefix' and '--full-path' for all sub_paths")
                .short("r")
                .long("recursive"),
        )
        .arg(
            Arg::with_name("no-clobber")
                .help("do not overwrite an existing file")
                .short("n")
                .long("no-clobber"),
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
        .get_matches()
}

fn convert_single(
    path: &Path,
    from: Option<Convention>,
    to: Convention,
    is_full_path: bool,
    prefix: Option<&Path>,
    is_verbose: bool,
    is_dry_run: bool,
    no_clobber: bool,
) -> Result<(), PathConvertError> {
    // todo: take this as closure rather than method
    //       store method or closure reference outside loop or run separate loops
    let new_path = if is_full_path {
        if prefix.is_some() {
            convert_path::convert_full_except_prefix(path, prefix.unwrap(), from, to)
        } else {
            convert_path::convert_full(path, from, to)
        }
    } else {
        convert_path::convert_basename(path, from, to)
    }?;

    if !is_dry_run {
        if new_path.exists() {
            if no_clobber {
                if is_verbose {
                    println!("file '{}' already exists", new_path.display());
                }

                return Ok(());
            }
        }

        let parent = new_path.parent();
        if parent.is_some() && !parent.unwrap().exists() {
            if let Err(err) = fs::create_dir_all(parent.unwrap()) {
                eprintln!("Error: {}", err);
                exit(4);
            }
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

    Ok(())
}

fn convert_recursive(
    dir: &Path,
    from: Option<Convention>,
    to: Convention,
    is_verbose: bool,
    is_dry_run: bool,
    no_clobber: bool,
) -> Result<(), PathConvertError> {
    for i in WalkDir::new(dir).contents_first(true) {
        if let Ok(entry) = i {
            let path = entry.path();

            convert_single(
                path, from, to, false, None, is_verbose, is_dry_run, no_clobber,
            )?;
        }
    }

    Ok(())
}

// todo: break this into smaller pieces
fn main() {
    let matches = get_matches();

    let is_verbose = matches.is_present("verbose");
    let is_dry_run = matches.is_present("dry-run");
    let no_clobber = matches.is_present("no-clobber");
    let is_recursive = matches.is_present("recursive");

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

    // ensure that all specified paths exist
    let paths: Vec<&Path> = matches
        .values_of("paths")
        .unwrap()
        .map(|v| {
            let path = Path::new(v);

            if !path.exists() {
                eprintln!("Error: no such file or directory '{}'", path.display());
                exit(2);
            }

            path
        })
        .collect();

    let is_full_path = matches.is_present("full-path");
    let prefix = if matches.is_present("prefix") {
        Some(Path::new(matches.value_of("prefix").unwrap()))
    } else {
        None
    };

    for path in paths {
        if path.is_dir() && is_recursive {
            convert_recursive(
                path,
                from_convention,
                to_convention,
                is_verbose,
                is_dry_run,
                no_clobber,
            );
        } else {
            convert_single(
                path,
                from_convention,
                to_convention,
                is_full_path,
                prefix,
                is_verbose,
                is_dry_run,
                no_clobber,
            );
        }
    }
}
