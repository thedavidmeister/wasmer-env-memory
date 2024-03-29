fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=*");

    let cargo_command = std::env::var_os("CARGO");
    let cargo_command = cargo_command
        .as_ref()
        .map(|s| &**s)
        .unwrap_or_else(|| "cargo".as_ref());

    let status = std::process::Command::new(cargo_command)
        .arg("build")
        .arg("--manifest-path")
        .arg("wasm/Cargo.toml")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &out_dir)
        .status()
        .unwrap();

    assert!(status.success());
}
