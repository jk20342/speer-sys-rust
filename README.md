# speer-sys

raw rust ffi for speer.

this crate generates bindings for the c headers and links `libspeer`.

## build

```bash
cargo check
```

by default it builds the c library from source with cmake. if you already have
speer installed:

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
