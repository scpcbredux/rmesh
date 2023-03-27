use rmesh::{read_rmesh, RMeshError};

fn main() -> Result<(), RMeshError> {
    let mut args = std::env::args();
    let _ = args.next();
    let bytes = std::fs::read(args.next().expect("No rmesh file provided")).unwrap();
    let rmesh = read_rmesh(&bytes)?;

    let mut index = 0;
    
    for mesh in rmesh.meshes {
        println!("Mesh {}", index);
        for texture in mesh.textures {
            if let Some(path) = texture.path {
                println!("\tTexture Path: {:#?}, {:#?}", String::from(path), texture.blend_type);
            }
        }
        index += 1;
    }

    Ok(())
}
