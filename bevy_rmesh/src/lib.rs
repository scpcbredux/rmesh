pub use loader::*;
pub use rmesh;

mod loader;

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{renderer::RenderDevice, texture::CompressedImageFormats},
};

#[derive(Default)]
pub struct RMeshPlugin;

impl Plugin for RMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Room>()
            .init_asset::<RoomMesh>()
            .preregister_asset_loader::<RMeshLoader>(&["rmesh"]);
    }

    fn finish(&self, app: &mut App) {
        let supported_compressed_formats = match app.world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::NONE,
        };
        app.register_asset_loader(RMeshLoader {
            supported_compressed_formats,
        });
    }
}

#[derive(Asset, Debug, TypePath)]
pub struct Room {
    pub scene: Handle<Scene>,
    pub meshes: Vec<RoomMesh>,
    // pub entity_meshes: Vec<RoomMesh>,
}

#[derive(Asset, Debug, TypePath)]
pub struct RoomMesh {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}
