extern crate clap;
extern crate rayon;

use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{App, Arg, ArgMatches};
use regex::Regex;
use subprocess::Exec;
use rayon::prelude::*;

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:#?}", e);
            1
        }
    });
}

// Execute action(terminal command/script) provide by user
fn run() -> Result<(), Box<dyn Error>> {
    let matches = get_args();

    let path = matches.value_of("PATH").unwrap();
    let action = matches.value_of("ACTION").unwrap();

    let paths = get_paths(&path);
    let paths = filter(&paths, &matches);
    perform_action(&paths, &action)?;

    Ok(())
}

// filter list of paths based on matches
fn filter(paths: &Vec<PathBuf>, matches: &ArgMatches) -> Vec<String> {
    let match_str = matches.value_of("MATCH").unwrap();
    let use_re = matches.is_present("regex");

    let re = if use_re {
        Some(Regex::new(match_str).unwrap())
    } else {
        None
    };

    paths
        .par_iter()
        .filter(|p| {
            let keep = false
                || (!matches.is_present("file") && !matches.is_present("directory"))
                || (matches.is_present("file") && matches.is_present("directory"))
                || (matches.is_present("file") && p.is_file())
                || (matches.is_present("directory") && p.is_dir());

            if !keep && matches.is_present("verbose") {
                eprintln!("sme: ignored: {}", p.as_path().to_str().unwrap());
            }
            keep
        })
        .map(|p| p.as_path().to_str().unwrap())
        .filter(|p| match &re {
            Some(re) => re.is_match(p),
            None => p.contains(match_str),
        })
        .map(|p| String::from(p))
        .collect::<Vec<String>>()
}

// Prepare action to be executed
fn perform_action(paths: &Vec<String>, action: &str) -> Result<(), Box<dyn Error>> {
    if action == "" {
        let out = paths
            .into_iter()
            .fold(String::new(), |acc, v| format!("{}\n{}", acc, &v));
        println!("{}", out);
    } else {
        for path in paths {
            let command = format!("{} {}", String::from(action), &path);
            Exec::shell(&command).join()?;
        }
    }

    Ok(())
}

// Traverse root path and find files
fn get_paths(path: &str) -> Vec<PathBuf> {
    let mut paths = vec![Path::new(path).to_path_buf()];
    let mut path_set = Vec::new();

    while let Some(path) = paths.pop() {
        let rd_path = path.as_path().read_dir();
        match rd_path {
            Err(e) => eprintln!(
                "{}:\n{}\n",
                e,
                path.as_path()
                    .to_str()
                    .expect("failed to convert path to str")
            ),
            Ok(val) => val.for_each(|p| match p {
                Err(e) => eprintln!("failed to get path: {:#?}", e),
                Ok(p) => {
                    if p.path().is_dir() {
                        paths.push(p.path().to_path_buf());
                    }
                    path_set.push(p.path().to_path_buf());
                }
            }),
        }
    }

    path_set
}

// CLI interface
fn get_args<'a>() -> ArgMatches<'a> {
    App::new("SME (Search match execute)")
        .version("1.0")
        .author("iv")
        .about("Small and fast path matcher with command execution.")
        .arg(
            Arg::with_name("MATCH")
                .help("match expression")
                .default_value("")
                .hide_default_value(true),
        )
        .arg(
            Arg::with_name("PATH")
                .help("directory to search")
                .default_value(".")
                .hide_default_value(true),
        )
        .arg(
            Arg::with_name("ACTION")
                .help("command line tool or program to exec on each file")
                .default_value("")
                .hide_default_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("regex")
                .help("use regular expressions for matching")
                .short("e")
                .long("regex"),
        )
        .arg(
            Arg::with_name("file")
                .help("match against at files")
                .short("f")
                .long("file"),
        )
        .arg(
            Arg::with_name("directory")
                .help("match against at files")
                .short("d")
                .long("dirs"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("output more information")
                .short("v")
                .long("verbose"),
        )
        .arg(
            Arg::with_name("depth")
                .help("recurse depth for search")
                .short("l")
                .long("depth")
                .takes_value(true),
        )
        .get_matches()
}
