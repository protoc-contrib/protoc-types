//! The `protoc.contrib.*` proto packages compiled for buffa: the options the
//! protoc-contrib plugins read, one module per package, at the path the
//! package names.
//!
//! - [`protoc::contrib::defaults`]: `protoc.contrib.defaults`, read by
//!   [protoc-gen-rust-defaults](https://github.com/protoc-contrib/protoc-gen-rust-defaults).
//! - [`protoc::contrib::json`]: `protoc.contrib.json`, read by
//!   [protoc-gen-go-json](https://github.com/protoc-contrib/protoc-gen-go-json).
//!
//! Generated once and committed, so depending on this crate needs no `protoc`.

#[allow(
    clippy::all,
    clippy::pedantic,
    warnings,
    reason = "buffa-generated code -- upstream codegen style, not ours to police"
)]
mod generated {
    include!("generated/_include.rs");
}

pub use generated::protoc;

#[cfg(test)]
mod tests {
    use super::protoc::contrib::{defaults, json};
    use buffa::Message;

    #[test]
    fn the_options_round_trip_through_the_wire() {
        let value = defaults::FieldDefaults {
            r#type: Some(defaults::field_defaults::Type::Int32(50)),
            ..defaults::FieldDefaults::default()
        };
        let decoded = defaults::FieldDefaults::decode_from_slice(&value.encode_to_vec()).unwrap();
        assert_eq!(
            decoded.r#type,
            Some(defaults::field_defaults::Type::Int32(50))
        );
    }

    #[test]
    fn the_extensions_are_numbered_as_the_protos_number_them() {
        assert_eq!(defaults::VALUE.number(), 55_001);
        assert_eq!(defaults::VALUE.extendee(), "google.protobuf.FieldOptions");
        assert_eq!(defaults::ONEOF.number(), 55_002);
        assert_eq!(defaults::SKIP.number(), 55_003);
        assert_eq!(defaults::UNEXPORTED.number(), 55_004);
        assert_eq!(json::SKIP.number(), 55_100);
        assert_eq!(json::SKIP.extendee(), "google.protobuf.MessageOptions");
        assert_eq!(json::ENUM_SKIP.number(), 55_100);
        assert_eq!(json::ENUM_SKIP.extendee(), "google.protobuf.EnumOptions");
    }
}
