fn main() {
    // trigger recompilation
    println!("cargo:rerun-if-env-changed=L10N_CONFIG_FILE");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    println!("cargo:rerun-if-changed=l10n.toml");
    println!("cargo:rerun-if-changed=config.toml");
}
