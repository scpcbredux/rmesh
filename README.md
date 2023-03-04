## RMesh

[![crates.io](https://img.shields.io/crates/v/rmesh.svg)](https://crates.io/crates/rmesh) [![docs.rs](https://img.shields.io/docsrs/v/rmesh.svg)](https://docs.rs/rmesh)

The game `SCPCB` uses the `.rmesh` extension for room meshes.

### Examples

```rust
let bytes = unimplemented!();

let rmesh = rmesh::RMesh::read(bytes)?;

let positions: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.position).collect();
let tex_coords: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.tex_coords).collect();

println!("Postions: {:#?}", positions);
println!("UVS: {:#?}", tex_coords);
```

### Task list

- [ ] Write documentation
- [ ] Create a writer
