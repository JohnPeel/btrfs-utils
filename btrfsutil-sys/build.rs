use std::{env, path::PathBuf};

use bindgen::{
    callbacks::{DeriveInfo, ParseCallbacks},
    Builder, CargoCallbacks,
};
use once_cell::sync::Lazy;

static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var("OUT_DIR").unwrap()));

#[derive(Debug)]
struct Tweaks;
impl ParseCallbacks for Tweaks {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        match info.name {
            "btrfs_util_subvolume_info" => vec!["Clone".to_string()],
            _ => Vec::new(),
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let library =
        pkg_config::probe_library("libbtrfsutil").expect("unable to find pkgconfig for btrfsutil");

    let clang_args = library
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.display()));

    Builder::default()
        .clang_args(clang_args)
        .header_contents("wrapper.h", "#include <btrfsutil.h>")
        .parse_callbacks(Box::new(CargoCallbacks))
        .parse_callbacks(Box::new(Tweaks))
        .use_core()
        .disable_name_namespacing()
        .blocklist_type("timespec")
        .allowlist_function("btrfs_.*")
        .allowlist_type("btrfs_.*")
        .allowlist_var("btrfs_.*")
        .prepend_enum_name(false)
        .generate()
        .expect("unable to generate bindings")
        .write_to_file(OUT_DIR.join("bindings.rs"))
        .expect("unable to write bindings")
}
