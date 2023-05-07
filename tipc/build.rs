use std::{path::PathBuf, env};

fn main() {
    let library_adapter = "ipc-native";
    let library_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let boost_dir = PathBuf::from(env::var_os("BOOST_LIB_DIR").unwrap());
    println!("cargo:rustc-link-lib=static={}", library_adapter);
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir]).unwrap().to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[boost_dir]).unwrap().to_str().unwrap()
    );
}