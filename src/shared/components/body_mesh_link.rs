use bevy::prelude::*;

#[derive(Component)]
pub struct BodyMeshLink {
    pub body_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
}