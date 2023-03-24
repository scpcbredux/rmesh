fn main() -> Result<(), rmesh::RMeshError> {
    let mut args = std::env::args();
    let _ = args.next();
    let bytes = std::fs::read(args.next().expect("No rmesh file provided"))?;
    let rmesh = rmesh::RMesh::read(&bytes)?;

    for texture in &rmesh.meshes[0].textures {
        println!("Texture Path: {}", texture.path);
    }

    Ok(())
}
