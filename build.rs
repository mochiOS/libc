use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=newlib/mod.rs");
    println!("cargo:rerun-if-changed=newlib/generic.rs");
    println!("cargo:rerun-if-changed=newlib/mochios.rs");

    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    );

    let upstream_path =
        manifest_dir.join(".generated/upstream-newlib-mod.rs");
    let generic_path =
        normalize_path(manifest_dir.join("newlib/generic.rs"));
    let mochios_path =
        normalize_path(manifest_dir.join("newlib/mochios.rs"));

    let mut source = fs::read_to_string(&upstream_path)
        .expect("failed to read staged upstream newlib/mod.rs");

    if !source.contains("mod generic;") {
        panic!("staged upstream source has no generic module declaration");
    }

    source = source.replacen(
        "mod generic;",
        &format!(
            "#[path = \"{generic_path}\"]\nmod generic;"
        ),
        1,
    );

    let arch_branch =
        "    } else if #[cfg(target_arch = \"arm\")] {";

    if !source.contains(arch_branch) {
        panic!("staged upstream source has no ARM architecture branch");
    }

    let mochios_branch = format!(
        concat!(
        "    }} else if #[cfg(target_os = \"mochios\")] {{\n",
        "        #[path = \"{}\"]\n",
        "        mod mochios;\n",
        "        pub use self::mochios::*;\n",
        "    }} else if #[cfg(target_arch = \"arm\")] {{"
        ),
        mochios_path,
    );

    source = source.replacen(
        arch_branch,
        &mochios_branch,
        1,
    );

    let out_dir = PathBuf::from(
        env::var_os("OUT_DIR").expect("OUT_DIR not set"),
    );

    fs::write(out_dir.join("newlib_mod.rs"), source)
        .expect("failed to write generated newlib_mod.rs");
}

fn normalize_path(path: PathBuf) -> String {
    path.display().to_string().replace('\\', "\\\\")
}