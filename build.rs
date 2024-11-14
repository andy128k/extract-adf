fn main() {
    cc::Build::new()
        .file("extract-adf.c")
        .compile("extract-adf-c");

    println!("cargo:rustc-link-lib=z");
}
