use std::{fs::File, io::Write};

use rmesh::{write_rmesh, ComplexMesh, Header, RMeshError, Vertex, ROOM_SCALE};

fn main() -> Result<(), RMeshError> {
    let mut args = std::env::args();
    let _ = args.next();

    let min_x = -1.0 / ROOM_SCALE;
    let min_y = -1.0 / ROOM_SCALE;
    let min_z = -1.0 / ROOM_SCALE;
    let max_x = 1.0 / ROOM_SCALE;
    let max_y = 1.0 / ROOM_SCALE;
    let max_z = 1.0 / ROOM_SCALE;

    let vertices = vec![
        // Front
        Vertex {
            position: [min_x, min_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, min_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, max_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, max_y, max_z],
            ..Default::default()
        },
        // Back
        Vertex {
            position: [min_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, min_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, min_y, min_z],
            ..Default::default()
        },
        // Right
        Vertex {
            position: [max_x, min_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, max_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, min_y, max_z],
            ..Default::default()
        },
        // Left
        Vertex {
            position: [min_x, min_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, max_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, min_y, min_z],
            ..Default::default()
        },
        // Top
        Vertex {
            position: [max_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, max_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, max_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, max_y, max_z],
            ..Default::default()
        },
        // Bottom
        Vertex {
            position: [max_x, min_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, min_y, max_z],
            ..Default::default()
        },
        Vertex {
            position: [min_x, min_y, min_z],
            ..Default::default()
        },
        Vertex {
            position: [max_x, min_y, min_z],
            ..Default::default()
        },
    ];

    let header = Header {
        meshes: vec![ComplexMesh {
            vertices,
            triangles: vec![
                [0, 1, 2],
                [2, 3, 0], // Front
                [4, 5, 6],
                [6, 7, 4], // Back
                [8, 9, 10],
                [10, 11, 8], // Right
                [12, 13, 14],
                [14, 15, 12], // Left
                [16, 17, 18],
                [18, 19, 16], // Top
                [20, 21, 22],
                [22, 23, 20], // Bottom
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let rmesh = write_rmesh(&header)?;
    let mut file = File::create(args.next().expect("No output location provided")).unwrap();
    file.write_all(&rmesh).unwrap();
    Ok(())
}
