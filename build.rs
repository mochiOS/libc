use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=newlib/mod.rs");
    println!("cargo:rerun-if-changed=newlib/generic.rs");
    println!("cargo:rerun-if-changed=newlib/mochios.rs");
    println!("cargo:rerun-if-changed=.generated/upstream-newlib-mod.rs");

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

    let pthread_type = "pub type pthread_t = c_ulong;";
    if !source.contains(pthread_type) {
        panic!("staged upstream source has no pthread_t declaration");
    }
    source = source.replacen(
        pthread_type,
        concat!(
            "#[cfg(target_os = \"mochios\")]\n",
            "pub type pthread_t = c_uint;\n",
            "#[cfg(not(target_os = \"mochios\"))]\n",
            "pub type pthread_t = c_ulong;"
        ),
        1,
    );

    // A local upstream definition takes precedence over the mochiOS glob
    // re-export. Select the newlib ABI here as well as in the platform module.
    let time64_branch = "target_os = \"horizon\",";
    if !source.contains(time64_branch) {
        panic!("staged upstream source has no expected time_t selection branch");
    }
    source = source.replacen(
        time64_branch,
        "target_os = \"horizon\",\n        target_os = \"mochios\",",
        1,
    );
    let microseconds_type = "pub type suseconds_t = i32;";
    assert!(source.contains(microseconds_type));
    source = source.replacen(
        microseconds_type,
        concat!(
            "#[cfg(target_os = \"mochios\")]\n",
            "pub type suseconds_t = c_long;\n",
            "#[cfg(not(target_os = \"mochios\"))]\n",
            "pub type suseconds_t = i32;"
        ),
        1,
    );

    let cloexec_branch = concat!(
        "if #[cfg(target_os = \"espidf\")] {\n",
        "        pub const O_CLOEXEC: c_int = 0x40000;"
    );
    if !source.contains(cloexec_branch) {
        panic!("staged upstream source has no expected O_CLOEXEC branch");
    }
    source = source.replacen(
        cloexec_branch,
        concat!(
            "if #[cfg(any(target_os = \"espidf\", target_os = \"mochios\"))] {\n",
            "        pub const O_CLOEXEC: c_int = 0x40000;"
        ),
        1,
    );

    source = source.replacen(
        "mod generic;",
        &format!(
            "#[path = \"{generic_path}\"]\nmod generic;"
        ),
        1,
    );

    // mochiOS stores mutex state inline; the generic newlib/Linux layouts
    // and byte-pattern initializer do not describe this target's C ABI.
    for declaration in ["pub struct pthread_mutex_t {", "pub struct pthread_mutexattr_t {",
        "pub struct pthread_cond_t {", "pub struct pthread_condattr_t {",
        "pub struct sem_t {", "pub struct pthread_rwlock_t {",
        "pub struct pthread_rwlockattr_t {"] {
        assert!(source.contains(declaration));
        source = source.replacen(declaration,
            &format!("#[cfg(not(target_os = \"mochios\"))]\n    {declaration}"), 1);
    }
    source = source.replacen("pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t =",
        "#[cfg(not(target_os = \"mochios\"))]\npub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t =", 1);
    source = source.replacen("pub const PTHREAD_COND_INITIALIZER: pthread_cond_t =",
        "#[cfg(not(target_os = \"mochios\"))]\npub const PTHREAD_COND_INITIALIZER: pthread_cond_t =", 1);
    source = source.replacen("pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t =",
        "#[cfg(not(target_os = \"mochios\"))]\npub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t =", 1);
    for (name, native, upstream_values) in [
        ("__SIZEOF_PTHREAD_RWLOCK_T", 4, &[4, 32, 56][..]),
        ("__SIZEOF_PTHREAD_RWLOCKATTR_T", 8, &[4, 8, 12][..]),
        ("__SIZEOF_PTHREAD_BARRIER_T", 32, &[4, 32][..]),
    ] {
        for value in upstream_values {
            let declaration = format!("pub const {name}: usize = {value};");
            let replacement = format!("pub const {name}: usize = if cfg!(target_os = \"mochios\") {{ {native} }} else {{ {value} }};");
            source = source.replace(&declaration, &replacement);
        }
    }
    for (declaration, replacement) in [
        ("pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;",
         "pub const __SIZEOF_PTHREAD_MUTEX_T: usize = if cfg!(target_os = \"mochios\") { 32 } else { 40 };"),
        ("pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;",
         "pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = if cfg!(target_os = \"mochios\") { 12 } else { 4 };"),
        ("pub const __SIZEOF_PTHREAD_COND_T: usize = 48;",
         "pub const __SIZEOF_PTHREAD_COND_T: usize = if cfg!(target_os = \"mochios\") { 24 } else { 48 };"),
        ("pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;",
         "pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = if cfg!(target_os = \"mochios\") { 12 } else { 4 };"),
    ] {
        assert!(source.contains(declaration));
        source = source.replace(declaration, replacement);
    }

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
