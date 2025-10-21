fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_i128)");
    println!("cargo:rustc-cfg=has_i128");
}
