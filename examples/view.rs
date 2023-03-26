use std::path::PathBuf;

use rmesh::{RMeshError, read_rmesh, Header};
use three_d::*;

pub const ROOM_SCALE: f32 = 8. / 2048.;

fn main() -> Result<(), RMeshError> {
    let mut args = std::env::args();
    let _ = args.next();
    let bytes = std::fs::read(args.next().expect("No rmesh file provided")).unwrap();
    let model = read_rmesh(&bytes)?;

    let window = Window::new(WindowSettings {
        title: "RMesh Viewer".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let cpu_meshes = get_header_meshes(&model);
    let ph_material = PhysicalMaterial {
        albedo: Color {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        },
        ..Default::default()
    };
    let material = CpuMaterial {
        albedo: Color {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        },
        ..Default::default()
    };

    let model: three_d::Model<PhysicalMaterial> = three_d::Model::new(
        &context,
        &CpuModel {
            materials: vec![material],
            geometries: cpu_meshes,
        },
    )
    .unwrap();

    let mut directional = [
        DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(1.0, -1.0, 0.0)),
        DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(1.0, 1.0, 0.0)),
    ];
    let mut ambient = AmbientLight {
        color: Color::WHITE,
        intensity: 0.2,
        ..Default::default()
    };

    // main loop
    window.render_loop(move |mut frame_input| {
        let viewport = Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let lights = &[&ambient as &dyn Light, &directional[0], &directional[1]];

        camera.set_perspective_projection(degrees(60.0), camera.z_near(), camera.z_far());

        let screen = frame_input.screen();
        let target = screen.clear(ClearState::default());
        let position_material = PositionMaterial::default();
        target.render_with_material(
            &position_material,
            &camera,
            model.iter().map(|gm| &gm.geometry),
            lights,
        );

        FrameOutput::default()
    });

    Ok(())
}

fn get_header_meshes(header: &Header) -> Vec<CpuMesh> {
    let mut cpu_meshes = vec![];

    for mesh in &header.meshes {
        let positions: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| Vector3::new(
                v.position[0] * ROOM_SCALE,
                v.position[1] * ROOM_SCALE,
                -v.position[2] * ROOM_SCALE,
            ))
            .collect();
        let indices = mesh
            .triangles
            .iter()
            .flat_map(|strip| strip.iter().rev().map(|index| *index as u32))
            .collect();

        cpu_meshes.push(CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        });
    }

    cpu_meshes
}
