fn main() {
    println!("cargo:rerun-if-changed=vector.S");
    cc::Build::new()
        .target("aarch64-unknown-none")
        .file("vector.S")
        .flag("-march=armv8-a")
        .compiler("aarch64-linux-gnu-gcc")
        .compile("vectors");
}
