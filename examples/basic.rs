use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use simply_collide::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(CollisionPluginConfig { debug_render: true })
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::Circle(50.0),
            ..ColliderBundle::default()
        })
        .insert(ColliderDebugRender { color: Color::RED });
}
