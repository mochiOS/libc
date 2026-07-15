use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=newlib/mod.rs");
    println!("cargo:rerun-if-changed=newlib/generic.rs");
    println!("cargo:rerun-if-changed=newlib/mochios.rs");

    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    copy_newlib_sources(&manifest_dir);

    let Some(upstream) = find_upstream_newlib_mod() else {
        panic!("failed to locate upstream libc newlib/mod.rs in Cargo registry");
    };

    let source =
        fs::read_to_string(&upstream).expect("failed to read upstream libc newlib/mod.rs");
    let generic_path =
        normalize_path(manifest_dir.join("src/unix/newlib/generic.rs"));
    let mochios_path =
        normalize_path(manifest_dir.join("src/unix/newlib/mochios.rs"));

    let source = source.replace(
        "mod generic;",
        &format!("#[path = \"{generic_path}\"]\nmod generic;"),
    );

    let needle = "} else if #[cfg(target_arch = \"aarch64\")] {";
    let replacement = format!(
        "}} else if #[cfg(target_os = \"mochios\")] {{\n        \
		#[path = \"{mochios_path}\"]\n        \
		mod mochios;\n        \
		pub use self::mochios::*;\n    \
		}} else if #[cfg(target_arch = \"aarch64\")] {{"
    );

    let patched = source.replace(needle, &replacement);
    if patched == source {
        panic!("failed to patch upstream libc newlib/mod.rs for mochios");
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(out_dir.join("newlib_mod.rs"), patched)
        .expect("failed to write generated newlib_mod.rs");
}

fn copy_newlib_sources(manifest_dir: &Path) {
    let source_dir = manifest_dir.join("newlib");
    let destination_dir = manifest_dir.join("src/unix/newlib");

    fs::create_dir_all(&destination_dir)
        .expect("failed to create src/unix/newlib");

    for name in ["mod.rs", "generic.rs", "mochios.rs"] {
        let source = source_dir.join(name);
        let destination = destination_dir.join(name);

        fs::copy(&source, &destination).unwrap_or_else(|error| {
            panic!(
                "failed to copy {} to {}: {error}",
                source.display(),
                destination.display(),
            )
        });
    }
}

fn find_upstream_newlib_mod() -> Option<PathBuf> {
    let cargo_home = env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| Path::new(&home).join(".cargo")))?;

    let registry_src = cargo_home.join("registry").join("src");
    let entries = fs::read_dir(registry_src).ok()?;

    for entry in entries.flatten() {
        let path = entry
            .path()
            .join("libc-0.2.185")
            .join("src")
            .join("unix")
            .join("newlib")
            .join("mod.rs");

        if path.is_file() {
            return Some(path);
        }
    }

    None
}

fn normalize_path(path: PathBuf) -> String {
    path.display().to_string().replace('\\', "\\\\")
}