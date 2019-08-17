//! `cargo-example` is an easy-to-use example runner for rust crates.
//!
//! # About
//!
//! `cargo-example` aims to provide a simple interface to running examples
//! of any valid rust crate. Currently there is no short way to do this.
//! In order to run examples you'll always have to manually clone or download
//! the repository of the crate whose examples you wish to run.  
//! For newcomers to the rust language the `--example` feature of `cargo run`
//! might even remain completely unknown for quite some time.  
//! This crate aims to simplify this process. All the messy cloning
//! is abstracted away from the end user.
//!
//! # Usage
//!
//! 1. Run `cargo install cargo-example`
//! 2. Run any examples with `cargo example <crate> ...args`
//! * `<crate>` is the target crate whose examples you wish to run
//! * `args` is a list of arguments that will be forwarded to the example  
//!
//! ## In development
//!
//! Just use `cargo run example <crate> ...args`. It has to be done this way due
//! to the way that cargo finds its subcommands.
//!
//! # Inspiration
//!
//! I wouldn't have made this project if it wasn't for the excellent GitHub
//! Project "request-for-implementation" by dtolnay. Specifically
//! [this](https://github.com/dtolnay/request-for-implementation/issues/30)
//! issue.
//!
//! # Early stages
//!
//! This entire project is still in its infancy. It might not work for you.
//! But you can help. Just open up a new GitHub issue if you find an error
//! or would like to request a feature or a change.

mod cache;
mod download;
mod error;
mod response;

use cache::*;
use clap::{App, AppSettings, Arg, SubCommand, Values};
use download::*;
use error::*;
use std::process::{Command, Stdio};

fn run_command(args: &Vec<&str>, path: &str) {
    let mut process = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .args(args)
        .current_dir(path)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Could not spawn a cargo process.");

    let exit_code = process.wait().expect("Could not wait for cargo to exit.");

    if !exit_code.success() {
        println!("Cargo did not exit successfully.");

        match exit_code.code() {
            Some(code) => println!("Status code: {}", code),
            None => println!("Reason: Terminated by signal"),
        }
    }
}

fn print_error(error: &Error) {
    error.errors.iter().for_each(|element| {
        println!("{}", element.detail);
    })
}

fn main() {
    let matches = App::new("cargo-example")
        .version("0.1.1")
        .bin_name("cargo")
        .settings(&[AppSettings::SubcommandRequired])
        .subcommand(
            SubCommand::with_name("example")
                .about("An easy example runner for cargo")
                .arg(Arg::with_name("project").required(true).index(1))
                .arg(Arg::with_name("additional_args").multiple(true)),
        )
        .get_matches();

    let subcommand_matches = matches.subcommand_matches("example").unwrap();
    let project = subcommand_matches.value_of("project").unwrap();
    let example_args: Vec<&str> = subcommand_matches
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
                print_error(&e);
            }
        }
    }
}
