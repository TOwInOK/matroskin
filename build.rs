fn main() {
    println!("cargo:rerun-if-changed=assets/doc");
    #[cfg_attr(docsrs, feature(doc_cfg))]
    std::fs::copy(".content/logo.webp", "target/doc/logo.ico")
        .expect("Failed to copy crate favicon when building documentation.");
    #[cfg_attr(docsrs, feature(doc_cfg))]
    std::fs::copy(".content/logo.webp", "target/doc/logo.webp")
        .expect("Failed to copy crate logo when building documentation.");
}
