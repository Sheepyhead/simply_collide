use bevy::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    pub shape: ColliderShape,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            shape: ColliderShape::Circle(10.0),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Component)]
pub enum ColliderShape {
    Circle(f32),
}
