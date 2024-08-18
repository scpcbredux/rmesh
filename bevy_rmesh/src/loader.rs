use std::path::Path;

use crate::{Room, RoomMesh};
use anyhow::Result;
use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::render::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};
use directx_mesh::read_directx_mesh;
use rmesh::{read_rmesh, ROOM_SCALE};
use serde::{Deserialize, Serialize};

pub struct RMeshLoader {
    pub(crate) supported_compressed_formats: CompressedImageFormats,
}

#[derive(Serialize, Deserialize)]
pub struct RMeshLoaderSettings {
    pub load_meshes: RenderAssetUsages,
    pub load_materials: RenderAssetUsages,
    pub load_entities: bool,
    pub load_lights: bool,
    pub load_xmeshes: bool,
}

impl Default for RMeshLoaderSettings {
    fn default() -> Self {
        Self {
            load_meshes: RenderAssetUsages::default(),
            load_materials: RenderAssetUsages::default(),
            load_entities: true,
            load_lights: true,
            load_xmeshes: true,
        }
    }
}

impl AssetLoader for RMeshLoader {
    type Asset = Room;
    type Settings = RMeshLoaderSettings;
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        settings: &'a RMeshLoaderSettings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        load_rmesh(self, &bytes, load_context, settings).await
    }

    fn extensions(&self) -> &[&str] {
        &["rmesh"]
    }
}

/// Loads an entire rmesh file.
async fn load_rmesh<'a, 'b, 'c>(
    loader: &RMeshLoader,
    bytes: &'a [u8],
    load_context: &'b mut LoadContext<'c>,
    settings: &'b RMeshLoaderSettings,
) -> Result<Room> {
    let header = read_rmesh(bytes)?;

    let mut meshes = vec![];
    let mut entity_meshes = vec![];

    for (i, complex_mesh) in header.meshes.iter().enumerate() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, settings.load_meshes);

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
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        let tex_uvs: Vec<_> = complex_mesh
            .vertices
            .iter()
            .map(|v| [v.tex_coords[0][0], v.tex_coords[0][1]])
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_uvs);

        let lightmaps_uvs: Vec<_> = complex_mesh
            .vertices
            .iter()
            .map(|v| [v.tex_coords[1][0], v.tex_coords[1][1]])
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, lightmaps_uvs);

        let indices = complex_mesh
            .triangles
            .iter()
            .flat_map(|strip| strip.iter().rev().copied())
            .collect();
        mesh.insert_indices(Indices::U32(indices));

        mesh.duplicate_vertices();
        mesh.compute_flat_normals();

        let mesh = load_context.add_labeled_asset(format!("Mesh{0}", i), mesh);

        // TODO: double_sided and crap
        let base_color_texture = if let Some(path) = &complex_mesh.textures[1].path {
            let texture = load_texture(
                &String::from(path),
                load_context,
                loader.supported_compressed_formats,
                settings.load_materials,
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
    if settings.load_xmeshes {
        for entity in &header.entities {
            if let Some(rmesh::EntityType::Model(data)) = &entity.entity_type {
                let name = &String::from(data.name.clone());
                let parent = load_context.path().parent().unwrap();
                let image_path = parent.join("props").join(name);
                let bytes = load_context.read_asset_bytes(image_path.clone()).await?;
                let content =
                    std::str::from_utf8(&bytes)?;

                let mesh = load_context
                    .add_labeled_asset(format!("EntityMesh{0}", name), load_x_mesh(content)?);
                entity_meshes.push(mesh);
            }
        }
    }

    let scene = {
        let mut world = World::default();
        let mut scene_load_context = load_context.begin_labeled_asset();

        world
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|parent| {
                if settings.load_entities {
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
                                    if !settings.load_lights {
                                        return;
                                    }

                                    parent.spawn(PointLightBundle {
                                        transform: Transform::from_translation(Vec3::new(
                                            data.position[0] * ROOM_SCALE,
                                            data.position[1] * ROOM_SCALE,
                                            -data.position[2] * ROOM_SCALE,
                                        )),
                                        point_light: PointLight {
                                            range: data.range,
                                            shadows_enabled: true,
                                            intensity: (data.intensity * 0.8).min(1.) * 60_00.,
                                            color: Color::srgb_u8(
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
                                    if !settings.load_lights {
                                        return;
                                    }

                                    parent.spawn(SpotLightBundle {
                                        transform: Transform::from_translation(Vec3::new(
                                            data.position[0] * ROOM_SCALE,
                                            data.position[1] * ROOM_SCALE,
                                            -data.position[2] * ROOM_SCALE,
                                        )),
                                        spot_light: SpotLight {
                                            range: data.range,
                                            shadows_enabled: true,
                                            intensity: (data.intensity * 0.8).min(1.) * 60_00.,
                                            color: Color::srgb_u8(
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
                                                -data.scale[1] * ROOM_SCALE,
                                                data.scale[2] * ROOM_SCALE,
                                            )
                                                .into(),
                                        },
                                        mesh: scene_load_context.get_label_handle(&mesh_label),
                                        ..Default::default()
                                    });
                                }
                                _ => (),
                            }
                        }
                    }
                }
            });

        let loaded_scene = scene_load_context.finish(Scene::new(world), None);
        load_context.add_loaded_labeled_asset("Scene", loaded_scene)
    };

    Ok(Room {
        scene,
        entity_meshes,
        meshes,
    })
}

/// Loads an entire x file.
fn load_x_mesh<'a>(content: &'a str) -> Result<Mesh> {
    let header = read_directx_mesh(content)?;

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions: Vec<_> = header.vertices.iter().map(|v| [v.0, -v.1, v.2]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let indices: Vec<u32> = header.faces.iter().flatten().cloned().collect();
    mesh.insert_indices(Indices::U32(indices));

    let normals: Vec<_> = header.normals.iter().map(|v| [v.0, v.1, v.2]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    Ok(mesh)
}

async fn load_texture<'a>(
    path: &str,
    load_context: &mut LoadContext<'a>,
    supported_compressed_formats: CompressedImageFormats,
    render_asset_usages: RenderAssetUsages,
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
        render_asset_usages,
    )?)
}
