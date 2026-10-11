# protoc-types

[![CI](https://github.com/protoc-contrib/protoc-types/actions/workflows/ci.yml/badge.svg)](https://github.com/protoc-contrib/protoc-types/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

The `protoc.contrib.*` proto packages: the options the protoc-contrib plugins
read, in one place, with their Go and Rust bindings.

A schema annotated for one plugin is often generated for several. A message
shared by a Go and a Rust service declares its defaults once, and both
[protoc-gen-go-defaults] and [protoc-gen-rust-defaults] read them. So the
options live here, not in any one plugin, and every plugin, every binding and
every schema uses the same copy.

[protoc-gen-go-defaults]: https://github.com/protoc-contrib/protoc-gen-go-defaults
[protoc-gen-rust-defaults]: https://github.com/protoc-contrib/protoc-gen-rust-defaults
[protoc-gen-go-json]: https://github.com/protoc-contrib/protoc-gen-go-json

## What is here

| Package | Read by | Go | Rust |
| --- | --- | --- | --- |
| `protoc.contrib.defaults` | [protoc-gen-go-defaults], [protoc-gen-rust-defaults] | `github.com/protoc-contrib/protoc-types/contrib/defaults` | `protoc_types::contrib::defaults` |
| `protoc.contrib.json` | [protoc-gen-go-json] | `github.com/protoc-contrib/protoc-types/contrib/json` | `protoc_types::contrib::json` |

The protos live under `proto/`, the module root, so schemas import them as
`protoc/contrib/...`. The Go and Rust paths mirror the package without its
`protoc.` prefix, which the repository name already carries.

## Using the options in a schema

The protos are published to the Buf Schema Registry as
`buf.build/protoc-contrib/protoc-types`:

```yaml
# buf.yaml
version: v2
deps:
  - buf.build/protoc-contrib/protoc-types
```

```proto
import "protoc/contrib/defaults/options.proto";

message ListBooksRequest {
  int32 page_size = 1 [(protoc.contrib.defaults.value).int32 = 50];
}
```

## Using the bindings

Go:

```bash
go get github.com/protoc-contrib/protoc-types
```

Rust, from GitHub, pinned to a `rev`: there are no crates.io releases, so a
branch would let the crate change under a build that did not ask it to.

```toml
[dependencies]
protoc-types = { git = "https://github.com/protoc-contrib/protoc-types", rev = "..." }
```

Generated code for both languages is committed, so neither needs `buf` or
`protoc` to build.

## Extension numbers

Every package extends the same `google.protobuf.*Options` messages, so its
numbers must not collide with another's on the same extendee.
`contrib/extensions_test.go` fails if they do.

| Range | Package |
| --- | --- |
| 55001–55004 | `protoc.contrib.defaults` |
| 55100 | `protoc.contrib.json` |

A new package takes the next free block of 100.

## Adding or changing a proto

```bash
nix develop
buf lint
buf generate                                    # Go bindings, in contrib/
UPDATE_GENERATED=1 cargo test --test generated  # Rust bindings, in src/generated
go test ./... && cargo test
```

Commit the generated code with the proto. CI fails if either is stale.

## License

[Apache 2.0](LICENSE)
