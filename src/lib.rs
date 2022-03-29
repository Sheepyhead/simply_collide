use bevy::prelude::*;
use prelude::*;

mod colliders;
mod systems;

#[derive(Clone, Copy, Default)]
pub struct CollisionPluginConfig {
    pub debug_render: bool,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(debug_render);
    }
}

pub mod prelude {
    pub use crate::colliders::*;
    pub use crate::systems::*;
    pub use crate::{CollisionPlugin, CollisionPluginConfig};
}
