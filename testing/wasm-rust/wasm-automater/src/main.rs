use std::process::Command;
use std::{io, fs};
use std::path::Path;

pub fn copy_dir(src: &Path, dest: &Path, file: Option<&str>) -> io::Result<()> {
    // Create the destination directory if it doesn't exist
    if !dest.exists() {
        fs::create_dir(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dest.join(&file_name);

        if file.is_some() && entry_path.is_dir(){
            continue;
        }

        if entry_path.is_dir() {
            // Recursively copy subdirectories
            copy_dir(&entry_path, &dest_path, file)?;
        } else{
            // Copy files
            if file.is_none() || ( file_name.to_str() == file) { 
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    }

    Ok(())
}

fn main() {

    let pwd= std::env::current_dir()
    .unwrap();
    
    Command::new("rustup")
    .current_dir(pwd.join("wasm-rust-src"))
    .args(["target", "add", "wasm32-wasi"])
    .spawn()
    .expect("adding wasm32-wasi rust toolchain command failed to start");
    
    Command::new("cargo")
    .current_dir(pwd.join("wasm-rust-src"))
    .args(["build", "--release", "--target", "wasm32-wasi"])
    .spawn()
    .expect("building wasm32 command failed to start");

    let src = pwd.join("wasm-rust-src/target/wasm32-wasi/release");
    let dest = pwd.join("workflow-wasm");

    copy_dir(&src, &dest, Some("wasm_rust_src.wasm")).unwrap();

}
// rustup target add wasm32-wasi

// cargo build --release --target wasm32-wasi 