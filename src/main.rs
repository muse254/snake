use bevy::{image::ImageSamplerDescriptor, prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::*;

static CELL_ROWS: u32 = 16;
static CELL_COLS: u32 = 13;

fn setup(mut cmd: Commands) {
    cmd.spawn((Camera2d, Transform::default()));
    spawn_grid_world(cmd);
}

fn spawn_apple(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn((
        Node {
            width: Val::Px(32.),
            height: Val::Px(32.),
            left: Val::Px(32.),
            top: Val::Px(32.),
            position_type: PositionType::Absolute,
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
    ));
}

fn spawn_grid_world(mut cmd: Commands) {
    let mut cells = Vec::new();
    for row in 1..=CELL_ROWS {
        for col in 1..=CELL_COLS {
            // info!("row: {}, col: {}", row, col);
            cells.push((
                Node {
                    width: Val::Px(32.),
                    height: Val::Px(32.),
                    left: Val::Px((col - 1) as f32 * 32.),
                    top: Val::Px((row - 1) as f32 * 32.),
                    border: UiRect::all(Val::Px(1.)),
                    position_type: PositionType::Absolute,
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
            ));
        }
    }

    cmd.spawn_batch(cells);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, (setup, spawn_apple))
        // .add_systems(EguiContextPass, inspector_ui)
        // .add_plugins(EguiPlugin {
        //     enable_multipass_for_primary_context: true,
        // })
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .run();
}

#[allow(dead_code)]
fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_inspector_egui::bevy_egui::EguiContext, With<PrimaryWindow>>()
        .single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    bevy_inspector_egui::bevy_egui::egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        bevy_inspector_egui::bevy_egui::egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            bevy_inspector_egui::bevy_egui::egui::CollapsingHeader::new("Materials").show(
                ui,
                |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(
                        world, ui,
                    );
                },
            );

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_entities(world, ui);
        });
    });
}
