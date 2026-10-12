//! Publishes the `proto/` include root to the crates that depend on this one.
//!
//! Their build scripts read it as `DEP_PROTOC_TYPES_PROTO` and pass it to
//! `protoc -I`, so a schema importing `protoc/contrib/*/options.proto` is
//! compiled against the same protos these bindings were generated from, at
//! the same rev, with no vendored copy to drift. Cargo hands the variable to
//! direct dependents only, so such a crate lists this one under
//! `[dependencies]`.
//!
//! It only prints a path: building this crate still needs no `protoc`.

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    println!("cargo::metadata=proto={manifest}/proto");
    println!("cargo::rerun-if-changed=build.rs");
}
