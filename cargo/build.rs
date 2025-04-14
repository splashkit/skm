use std::env;
use std::path::PathBuf;

fn main() {
    let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
    let username = env::var("USER").unwrap_or_else(|_| String::from(""));
    
    // Define search paths based on OS
    let search_paths = match env::consts::OS {
        "macos" => vec![
            PathBuf::from("/usr/local/lib"),
            PathBuf::from(&home).join(".splashkit/lib/macos"),
        ],
        "linux" => vec![
            PathBuf::from("/usr/local/lib"),
            PathBuf::from(&home).join(".splashkit/lib/linux"),
        ],
        "windows" => vec![
            PathBuf::from(env::var("SYSTEMROOT").unwrap_or(String::from("C:/Windows"))).join("System32"),
            PathBuf::from(format!("C:/msys64/home/{}", username)).join(".splashkit/lib/win64"),
        ],
        _ => panic!("Unsupported operating system")
    };

    // Add all search paths
    for path in search_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Link against SplashKit
    println!("cargo:rustc-link-lib=dylib=SplashKit");

    // Rebuild if this build script changes
    println!("cargo:rerun-if-changed=build.rs");
}