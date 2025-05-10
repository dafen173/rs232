// fn main() {
//     tauri_build::build()
// }


fn main() {
    // Disable default resource generation
    println!("cargo:rerun-if-changed=build.rs");
    
    // Only enable on Windows native builds
    if std::env::var("TARGET").unwrap().contains("windows") {
        tauri_build::build();
    }
}

