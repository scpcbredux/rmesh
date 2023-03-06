## RMesh

[![crates.io](https://img.shields.io/crates/v/rmesh.svg)](https://crates.io/crates/rmesh) [![docs.rs](https://docs.rs/rmesh/badge.svg)](https://docs.rs/rmesh)

Rust parser for `.rmesh` file. The `.rmesh` is used in `SCPCB` for room meshes.

### Examples

```rust
fn main() -> Result<(), rmesh::RMeshError> {
    let bytes = std::fs::read("GFX/map/checkpoint1_opt.rmesh")?;
    let rmesh = rmesh::RMesh::read(&bytes)?;
    println!("{:#?}", rmesh);

    Ok(())
}
```

### Task list

- [ ] Write documentation
- [ ] Create a writer
