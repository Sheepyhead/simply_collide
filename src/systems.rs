use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{prelude::*, CollisionPluginConfig};

pub(crate) fn setup(mut commands: Commands, config: Option<Res<CollisionPluginConfig>>) {
    commands.insert_resource(match config {
        Some(config) => *config,
        None => CollisionPluginConfig::default(),
    });
}

pub(crate) fn debug_render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Option<Res<CollisionPluginConfig>>,
    colliders: Query<(Entity, &ColliderShape, &ColliderDebugRender)>,
) {
    if let Some(config) = config {
        if config.is_changed() {
            if config.debug_render {
                for (entity, collider, render) in colliders.iter() {
                    let child = commands
                        .spawn_bundle(match *collider {
                            ColliderShape::Circle(radius) => MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(Mesh::from(shape::Icosphere {
                                        radius,
                                        subdivisions: 8.0 as usize,
                                    }))
                                    .into(),
                                material: materials.add(ColorMaterial::from(render.color)),
                                ..MaterialMesh2dBundle::default()
                            },
                        })
                        .id();
                    commands.entity(entity).add_child(child);
                }
            } else {
                for (entity, _, _) in colliders.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

#[derive(Component)]
pub struct ColliderDebugRender {
    pub color: Color,
}
