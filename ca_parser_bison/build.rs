fn main() {
    match process_bison_file(Path::new("src/parser.y")) {
        Ok(_) => {}
        Err(BisonErr { message, .. }) => {
            eprintln!("Bison error:\n{}\nexiting with 1", message);
            std::process::exit(1);
        }
    }
    println!("cargo:rerun-if-changed=src/parser.y");
}

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct BisonErr {
    pub message: String,
    pub code: Option<i32>,
}

impl fmt::Display for BisonErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BisonErr: {:#?} ({:#?})", self.message, self.code)
    }
}

impl Error for BisonErr {}

/// Creates a `.rs` file from the given `.y` file
/// Output file is created in the same directory
pub fn process_bison_file(filepath: &Path) -> Result<(), BisonErr> {
    let input = filepath;
    let output = filepath.with_extension("rs");
    // let out_dir = std::env::var_os("OUT_DIR").unwrap();
    // let mut output = PathBuf::from(&out_dir);
    // output.push("out.rs");

    let bison_root_dir = PathBuf::new().join("bison");
    println!("Bsion root dir {:?}", bison_root_dir);
    let bison_root_file = bison_root_dir.join("main.m4");

    let args = &[
        "-S",
        bison_root_file.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        input.to_str().unwrap(),
    ];

    let output = Command::new("bison").args(args).output().unwrap();

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        Err(BisonErr {
            message: stderr,
            code: output.status.code(),
        })
    }
}
