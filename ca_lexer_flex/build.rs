use std::{path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=lex.h");
    println!("cargo:rerun-if-changed=lex.yy.c");

    Command::new("flex")
        .arg("--header-file=lex.h")
        .arg("src/lex.l")
        .spawn()
        .unwrap();
    cc::Build::new()
        .compiler("clang")
        .file("lex.yy.c")
        // .define("YY_INPUT(b,r,s)", "readInputForLexer(b,&r,s)")
        .compile("nlex");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
