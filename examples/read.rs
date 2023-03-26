use rmesh::{read_rmesh, RMeshError};

fn main() -> Result<(), RMeshError> {
    let mut args = std::env::args();
    let _ = args.next();
    let bytes = std::fs::read(args.next().expect("No rmesh file provided")).unwrap();
    let rmesh = read_rmesh(&bytes)?;
    println!("RMesh: {:#?}", rmesh);
    Ok(())
}
