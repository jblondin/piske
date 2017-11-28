
use std::io::{Read, Write};
use std::fs::File;

extern crate piske;

fn interpret_file(file_name: &str) {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut source = String::new();
            match file.read_to_string(&mut source) {
                Ok(_) => {
                    match piske::interpret::interpret(&source) {
                        Ok(_) => {},
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
    if args.len() != 2 {
        writeln!(::std::io::stderr(), "Usage: {} {}", args[0], args[1]).unwrap();
        ::std::process::exit(1);
    }
    interpret_file(&args[1]);
}