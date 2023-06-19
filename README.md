## RMesh

[![crates.io](https://img.shields.io/crates/v/rmesh.svg)](https://crates.io/crates/rmesh) [![docs.rs](https://docs.rs/rmesh/badge.svg)](https://docs.rs/rmesh)

Rust parser for **rmesh** file extension. The **rmesh** file extension is used in `SCPCB` for room meshes.

### Usage

```rust
let bytes = std::fs::read("GFX/map/lockroom_opt.rmesh").unwrap();
let rmesh = read_rmesh(&bytes).unwrap();
assert_eq!(rmesh.colliders.len(), 0);
assert_eq!(rmesh.entities.len(), 13);
```

### Examples

- [read](rmesh/examples/read.rs)
- [write](rmesh/examples/write.rs)
- [bevy_rmesh](bevy_rmesh/examples/view.rs)

### Task list

- [ ] Write documentation
- [X] Create a writer
- [ ] Add X file loader for bevy_rmesh
