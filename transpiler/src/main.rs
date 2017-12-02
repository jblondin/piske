extern crate cargo;

use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::{OpenOptions, File};

extern crate piske;

fn create_project_dir(proj_dir: &str) -> PathBuf {
    let new_opts = cargo::ops::NewOptions::new(None, true, false, proj_dir, None);
    let config = cargo::util::Config::default().unwrap();
    cargo::ops::new(&new_opts, &config).unwrap();
    config.cwd().join(new_opts.path)
}

macro_rules! try_file {
    ($attempt:expr, mut $var:ident, $success:block) => {{
        match $attempt {
            Ok(mut $var) => { $success },
            Err(e) => {
                writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                ::std::process::exit(1);
            }
        }
    }};
    ($attempt:expr, $var:ident, $success:block) => {{
        match $attempt {
            Ok($var) => { $success },
            Err(e) => {
                writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                ::std::process::exit(1);
            }
        }
    }};
    ($attempt:expr, $success:block) => {{
        match $attempt {
            Ok(_) => { $success },
            Err(e) => {
                writeln!(::std::io::stderr(), "file error: {}", e).unwrap();
                ::std::process::exit(1);
            }
        }
    }}
}

fn transpile_file(file_name: &str, proj_dir: &str) {
    let proj_dir = create_project_dir(proj_dir);

    let transpiled = try_file!(File::open(file_name), mut file, {
        let mut source = String::new();
        try_file!(file.read_to_string(&mut source), {
            match piske::glue::transpile(&source) {
                Ok(tokens) => tokens,
                Err(e) => {
                    writeln!(::std::io::stderr(), "interpreting failed: {}", e).unwrap();
                    ::std::process::exit(1);
                }
            }
        })

    });

    // write main script
    try_file!(File::create(proj_dir.join("src/main.rs")), mut f, {
        try_file!(f.write_all(transpiled.as_str().as_bytes()), {});
    });

    // write Cargo.toml
    try_file!(OpenOptions::new().append(true).open(proj_dir.join("Cargo.toml")), mut f, {
        try_file!(f.write_all(r#"
psk_std = { path = "../piske/psk_std" }
image = "*"
        "#.as_bytes()), {});
    });

}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        writeln!(::std::io::stderr(), "Usage: {} <file> <project_path>", args[0]).unwrap();
        ::std::process::exit(1);
    }
    transpile_file(&args[1], &args[2]);
}
