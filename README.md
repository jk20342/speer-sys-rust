# speer-sys

raw rust ffi for speer.

this crate generates bindings for the c headers and links `libspeer`.

## build

```bash
cargo check
```

by default it builds the c library for you with cmake. users do not need to
manually build speer first, but they do need the native build tools:

- cmake
- a c compiler
- clang/libclang for bindgen
- pkg-config if linking an installed copy

the c source is found from `SPEER_SOURCE_DIR` when set, otherwise from
`../../speer` in this repo layout.

if you already have speer installed:

```bash
SPEER_INCLUDE_DIR=/path/to/include SPEER_LIB_DIR=/path/to/lib cargo check --no-default-features
```

## features

- `build-from-source` - build `libspeer` from the c repo with cmake
- `static` - link `libspeer` statically
- `libp2p-tcp` - bind `speer_libp2p_tcp.h`
- `full-chat` - bind the extra tcp, mdns, multistream, protobuf, varint,
  ed25519, noise, and yamux surface used by `speer-chat`

full chat bindings:

```bash
cargo check --features full-chat
```

this is intentionally low-level. most rust code should depend on `speer`
instead.
