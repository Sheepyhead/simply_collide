use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
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
        .insert_bundle((ColliderDebugRender { color: Color::RED }, Player));
}

#[derive(Component)]
struct Player;

fn input(mut events: EventReader<KeyboardInput>, mut player: Query<&mut Transform, With<Player>>) {
    for event in events.iter() {
        let transform = player.single_mut();
        match event {
            KeyboardInput {
                key_code: Some(KeyCode::E),
                state: ElementState::Pressed,
                ..
            } => {}
            KeyboardInput {
                key_code: Some(KeyCode::O),
                state: ElementState::Pressed,
                ..
            } => {}
            KeyboardInput {
                key_code: Some(KeyCode::Comma),
                state: ElementState::Pressed,
                ..
            } => {}
            KeyboardInput {
                key_code: Some(KeyCode::A),
                state: ElementState::Pressed,
                ..
            } => {}
            _ => {}
        }
    }
}
