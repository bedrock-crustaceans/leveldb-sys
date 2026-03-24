fn main() {
    let mut config = cmake::Config::new("ffi");
    let mut ffi_dst = config.build().join("build");
    let mut leveldb_dst = ffi_dst.join("leveldb");

    if cfg!(target_env = "msvc") {
        let profile = config.get_profile();
        ffi_dst = ffi_dst.join(profile);
        leveldb_dst = leveldb_dst.join(profile);

        println!("cargo:rustc-link-lib=shell32");
    }

    println!(
        "Searching for leveldb-ffi and leveldb-mcpe in {}",
        ffi_dst.display()
    );
    println!("cargo:rustc-link-search=native={}", ffi_dst.display());
    println!("cargo:rustc-link-search=native={}", leveldb_dst.display());
    println!("cargo:rustc-link-lib=static=leveldb-ffi");
    println!("cargo:rustc-link-lib=static=leveldb-mcpe");

    #[cfg(unix)]
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
