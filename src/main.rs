mod cache;
mod download;
mod error;
mod response;

use cache::*;
use clap::{App, Arg, Values};
use download::*;
use std::process::{Command, Stdio};

fn run_command(args: &Vec<&str>, path: &str) {
    Command::new("cargo")
        .arg("run")
        .arg("--example")
        .args(args)
        .current_dir(path)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
}

fn main() {
    let matches = App::new("cargo-example")
        .version("0.1")
        .about("An easy example runner for cargo")
        .arg(Arg::with_name("project").required(true).index(1))
        .arg(Arg::with_name("additional_args").multiple(true))
        .get_matches();

    let project = matches.value_of("project").unwrap();
    let example_args: Vec<&str> = matches
        .values_of("additional_args")
        .unwrap_or(Values::default())
        .collect();

    let cache = try_load_from_cache(&project);
    let mut cache_path: Option<String>;

    match cache {
        Ok(repo) => cache_path = repository_to_path(repo),
        Err(_) => cache_path = None,
    }

    if let Some(example_path) = cache_path {
        run_command(&example_args, &example_path);
    } else {
        let download_result = download(project);

        match download_result {
            Ok(example_path) => run_command(&example_args, &example_path),
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
