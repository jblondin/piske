extern crate cargo;

use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::File;

extern crate piske;

fn create_project_dir(proj_dir: &str) -> PathBuf {
    let new_opts = cargo::ops::NewOptions::new(None, true, false, proj_dir, None);
    let config = cargo::util::Config::default().unwrap();
    cargo::ops::new(&new_opts, &config).unwrap();
    config.cwd().join(new_opts.path)
}

fn transpile_file(file_name: &str, proj_dir: &str) {
    let proj_dir = create_project_dir(proj_dir);
    let mut main_rs = match File::create(proj_dir.join("src/main.rs")) {
        Ok(f) => f,
        Err(e) => {
            writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
            ::std::process::exit(1);
         }
    };

    match File::open(file_name) {
        Ok(mut file) => {
            let mut source = String::new();
            match file.read_to_string(&mut source) {
                Ok(_) => {
                    match piske::glue::transpile(&source) {
                        Ok(tokens) => {
                            match main_rs.write_all(tokens.as_str().as_bytes()) {
                                Ok(_) => {},
                                Err(e) => {
                                    writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                                    ::std::process::exit(1);
                                }
                            }
                        },
                        Err(e) => {
                            writeln!(::std::io::stderr(), "interpreting failed: {}", e).unwrap();
                            ::std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                    ::std::process::exit(1);
                }
            }
        },
        Err(e) => {
            writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
            ::std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        writeln!(::std::io::stderr(), "Usage: {} <file> <project_path>", args[0]).unwrap();
        ::std::process::exit(1);
    }
    transpile_file(&args[1], &args[2]);
}
