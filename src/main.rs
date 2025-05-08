use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_aseprite_ultra::prelude::*;

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((Camera2d, Transform::default()));

    cmd.spawn((
        Node {
            width: Val::Px(32.),
            height: Val::Px(32.),
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        AseUiSlice {
            name: "apple".into(),
            aseprite: server.load("apple.aseprite"),
        },
        Sprite {
            flip_x: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(32., 0., 0.)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .run();
}
