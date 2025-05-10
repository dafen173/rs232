fn main() {
    // Only enable resource generation on native Windows builds
    if std::env::var("TARGET").unwrap().contains("windows") {
        tauri_build::build();
    } else {
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }
}