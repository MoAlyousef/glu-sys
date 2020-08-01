fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => println!("cargo:rustc-link-lib=framework=OpenGL"),
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=opengl32");
            println!("cargo:rustc-link-lib=dylib=glu32");
        },
        _ => {
                println!("cargo:rustc-link-lib=dylib=GL");
                println!("cargo:rustc-link-lib=dylib=GLU");
        },

    }
}
