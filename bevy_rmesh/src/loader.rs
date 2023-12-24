use std::path::Path;

use crate::{Room, RoomMesh};
use anyhow::Result;
use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::texture::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::render::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};
use rmesh::{read_rmesh, ROOM_SCALE};

pub struct RMeshLoader {
    pub(crate) supported_compressed_formats: CompressedImageFormats,
}

impl AssetLoader for RMeshLoader {
    type Asset = Room;
    type Settings = ();
    type Error = anyhow::Error;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Room, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            load_rmesh(self, &bytes, load_context).await
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rmesh"]
    }
}

/// Loads an entire rmesh file.
async fn load_rmesh<'a, 'b>(
    loader: &RMeshLoader,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<Room> {
    let header = read_rmesh(bytes)?;

    let mut meshes = vec![];
    // let mut entity_meshes = vec![];

    for (i, complex_mesh) in header.meshes.iter().enumerate() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let positions: Vec<_> = complex_mesh
            .vertices
            .iter()
            .map(|v| {
                [
                    v.position[0] * ROOM_SCALE,
                    v.position[1] * ROOM_SCALE,
                    -v.position[2] * ROOM_SCALE,
                ]
            })
            .collect();

        let tex_coords: Vec<_> = complex_mesh
            .vertices
            .iter()
            .flat_map(|v| {
                [
                    [v.tex_coords[0][0], 1.0 - v.tex_coords[0][1]], // First UV channel
                    [v.tex_coords[1][0], 1.0 - v.tex_coords[1][1]], // Second UV channel
                ]
            })
            .collect();
        let indices = complex_mesh
            .triangles
            .iter()
            .flat_map(|strip| strip.iter().rev().copied())
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();

        let mesh = load_context.add_labeled_asset(format!("Mesh{0}", i), mesh);

        let base_color_texture = if let Some(path) = &complex_mesh.textures[1].path {
            let texture = load_texture(
                &String::from(path),
                load_context,
                loader.supported_compressed_formats,
            )
            .await?;
            Some(load_context.add_labeled_asset(format!("Texture{0}", i), texture))
        } else {
            None
        };

        let material = load_context.add_labeled_asset(
            format!("Material{0}", i),
            StandardMaterial {
                base_color_texture,
                ..Default::default()
            },
        );

        meshes.push(RoomMesh { mesh, material });
    }

    // TODO: add setting if we want to load models with "x"
    // for entity in &header.entities {
    //     if let Some(rmesh::EntityType::Model(data)) = &entity.entity_type {
    //         let name = &String::from(data.name.clone());
    //         let (mesh, tex_path) = load_xfile(name, load_context).await?;

    //         let mesh_handle = load_context
    //             .set_labeled_asset(&format!("EntityMesh{0}", name), LoadedAsset::new(mesh));

    //         let base_color_texture = {
    //             let texture = load_xtexture(&tex_path, load_context).await?;
    //             Some(load_context.set_labeled_asset(
    //                 &format!("EntityTexture{0}", name),
    //                 LoadedAsset::new(texture),
    //             ))
    //         };

    //         let material_handle = load_context.set_labeled_asset(
    //             &format!("EntityMaterial{0}", name),
    //             LoadedAsset::new(StandardMaterial {
    //                 base_color_texture,
    //                 ..Default::default()
    //             }),
    //         );

    //         entity_meshes.push(RoomMesh {
    //             mesh: mesh_handle,
    //             material: material_handle,
    //         });
    //     }
    // }

    let scene = {
        let mut world = World::default();
        let mut scene_load_context = load_context.begin_labeled_asset();

        world
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|parent| {
                for i in 0..header.meshes.len() {
                    let mesh_label = format!("Mesh{0}", i);
                    let mat_label = format!("Material{0}", i);
                    let mut mesh_entity = parent.spawn(PbrBundle {
                        mesh: scene_load_context.get_label_handle(&mesh_label),
                        material: scene_load_context.get_label_handle(&mat_label),
                        ..Default::default()
                    });
                    let complex_mesh = &header.meshes[i];
                    if let Some((min, max)) = rmesh::calculate_bounds(&complex_mesh.vertices) {
                        mesh_entity.insert(Aabb::from_min_max(
                            Vec3::from_slice(&min),
                            Vec3::from_slice(&max),
                        ));
                    }
                }
                for entity in header.entities {
                    if let Some(entity_type) = entity.entity_type {
                        match entity_type {
                            rmesh::EntityType::Light(data) => {
                                parent.spawn(PointLightBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        data.position[0] * ROOM_SCALE,
                                        data.position[1] * ROOM_SCALE,
                                        -data.position[2] * ROOM_SCALE,
                                    )),
                                    point_light: PointLight {
                                        range: data.range,
                                        shadows_enabled: true,
                                        intensity: (data.intensity * 0.8).min(1.) * 60.,
                                        color: Color::rgb_u8(
                                            data.color.0[0],
                                            data.color.0[1],
                                            data.color.0[2],
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            rmesh::EntityType::SpotLight(data) => {
                                parent.spawn(SpotLightBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        data.position[0] * ROOM_SCALE,
                                        data.position[1] * ROOM_SCALE,
                                        -data.position[2] * ROOM_SCALE,
                                    )),
                                    spot_light: SpotLight {
                                        range: data.range,
                                        shadows_enabled: true,
                                        intensity: (data.intensity * 0.8).min(1.) * 60.,
                                        color: Color::rgb_u8(
                                            data.color.0[0],
                                            data.color.0[1],
                                            data.color.0[2],
                                        ),
                                        inner_angle: data.inner_cone_angle,
                                        outer_angle: data.outer_cone_angle,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            rmesh::EntityType::Model(data) => {
                                let name = &String::from(data.name.clone());
                                let mesh_label = format!("EntityMesh{0}", name);
                                let mat_label = format!("EntityMaterial{0}", name);

                                parent.spawn(PbrBundle {
                                    transform: Transform {
                                        translation: (
                                            data.position[0] * ROOM_SCALE,
                                            data.position[1] * ROOM_SCALE,
                                            -data.position[2] * ROOM_SCALE,
                                        )
                                            .into(),
                                        rotation: Quat::from_euler(
                                            EulerRot::XYZ,
                                            data.rotation[0],
                                            data.rotation[1],
                                            data.rotation[2],
                                        ),
                                        scale: (
                                            data.scale[0] * ROOM_SCALE,
                                            data.scale[1] * ROOM_SCALE,
                                            data.scale[2] * ROOM_SCALE,
                                        )
                                            .into(),
                                    },
                                    mesh: scene_load_context.get_label_handle(&mesh_label),
                                    material: scene_load_context.get_label_handle(&mat_label),
                                    ..Default::default()
                                });
                            }
                            _ => (),
                        }
                    }
                }
            });

        let loaded_scene = scene_load_context.finish(Scene::new(world), None);
        load_context.add_loaded_labeled_asset("Scene", loaded_scene)
    };

    Ok(Room {
        scene,
        // entity_meshes,
        meshes,
    })
}

async fn load_texture<'a>(
    path: &str,
    load_context: &mut LoadContext<'a>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<Image> {
    let parent = load_context.path().parent().unwrap();
    let image_path = parent.join(path);
    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

    let extension = Path::new(path).extension().unwrap().to_str().unwrap();
    let image_type = ImageType::Extension(extension);

    Ok(Image::from_buffer(
        &bytes,
        image_type,
        supported_compressed_formats,
        true,
        ImageSampler::Default,
    )?)
}
