pub fn main() {
    let mut cc = cc::Build::new();
    cc.file("src/test.c");
    let path = std::env::var("DEP_JPEG_INCLUDE").expect("paths");
    cc.includes(std::env::split_paths(&path));
    cc.compile("test");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=test");
    println!("cargo:rustc-link-lib=static=mozjpeg80");
}
