use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=SPEER_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=SPEER_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SPEER_SOURCE_DIR");
    println!("cargo:rerun-if-changed=wrapper.h");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let speer_root = env::var_os("SPEER_SOURCE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("..").join("..").join("speer"));
    let source_include = speer_root.join("include");

    let mut include_paths = Vec::new();
    if let Some(include_dir) = env::var_os("SPEER_INCLUDE_DIR") {
        include_paths.push(PathBuf::from(include_dir));
    }

    if let Some(lib_dir) = env::var_os("SPEER_LIB_DIR") {
        println!(
            "cargo:rustc-link-search=native={}",
            PathBuf::from(lib_dir).display()
        );
        link_speer();
    } else if let Some(dst) = build_speer_from_source(&speer_root) {
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib64").display()
        );
        link_speer();
        include_paths.push(dst.join("include"));
    } else if let Ok(library) = pkg_config::Config::new().probe("speer") {
        include_paths.extend(library.include_paths);
    } else {
        link_speer();
    }

    if include_paths.is_empty() {
        include_paths.push(source_include.clone());
    }

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("speer_.*")
        .allowlist_function("mdns_.*")
        .allowlist_type("speer_.*")
        .allowlist_type("mdns_.*")
        .allowlist_var("SPEER_.*")
        .allowlist_var("MDNS_.*")
        .allowlist_var("MULTISTREAM_.*")
        .allowlist_var("PB_WIRE_.*")
        .blocklist_function("speer_config_default")
        .generate_comments(true);

    for include_path in &include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
    }

    if cfg!(feature = "libp2p-tcp") {
        builder = builder.clang_arg("-DSPEER_SYS_BIND_LIBP2P_TCP");
        builder = add_source_include_dirs(builder, &speer_root);
    }

    if cfg!(feature = "full-chat") {
        builder = builder.clang_arg("-DSPEER_SYS_BIND_FULL_CHAT");
        builder = add_source_include_dirs(builder, &speer_root);
    }

    let bindings = builder
        .generate()
        .expect("failed to generate speer bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write speer bindings");
}

#[cfg(feature = "build-from-source")]
fn build_speer_from_source(speer_root: &Path) -> Option<PathBuf> {
    Some(
        cmake::Config::new(speer_root)
            .define("SPEER_BUILD_TESTS", "OFF")
            .define("SPEER_BUILD_EXAMPLES", "OFF")
            .define("SPEER_BUILD_TOOLS", "OFF")
            .define("SPEER_BUILD_FUZZ", "OFF")
            .define("SPEER_BUILD_BENCHMARK", "OFF")
            .define("SPEER_BUILD_DOCS", "OFF")
            .build(),
    )
}

#[cfg(not(feature = "build-from-source"))]
fn build_speer_from_source(_speer_root: &Path) -> Option<PathBuf> {
    None
}

fn link_speer() {
    if cfg!(feature = "static") || cfg!(feature = "build-from-source") {
        println!("cargo:rustc-link-lib=static=speer");
    } else {
        println!("cargo:rustc-link-lib=speer");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=iphlpapi");
        println!("cargo:rustc-link-lib=advapi32");
    } else {
        println!("cargo:rustc-link-lib=m");
    }
}

fn add_source_include_dirs(mut builder: bindgen::Builder, speer_root: &Path) -> bindgen::Builder {
    let src = speer_root.join("src");
    let dirs = [
        src.clone(),
        src.join("crypto"),
        src.join("discovery"),
        src.join("infra"),
        src.join("libp2p"),
        src.join("quic"),
        src.join("relay"),
        src.join("tls"),
        src.join("transport"),
        src.join("util"),
        src.join("wire"),
    ];

    for dir in dirs {
        builder = builder.clang_arg(format!("-I{}", dir.display()));
    }

    builder
}
