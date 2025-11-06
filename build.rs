fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets/doc");
    std::fs::copy(".content/logo.webp", "target/doc/logo.ico")
        .expect("Failed to copy crate favicon when building documentation.");
    std::fs::copy(".content/logo.webp", "target/doc/logo.webp")
        .expect("Failed to copy crate logo when building documentation.");
}
