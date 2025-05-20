use bevy::{image::ImageSamplerDescriptor, prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::*;

mod apple;
mod magic_numbers;
mod snake;
mod state_events;
use apple::Apple;
use magic_numbers::*;
use snake::{Direction, Snake, SnakeRenderMarker};
use state_events::GameEvent;

fn setup(mut cmd: Commands) {
    cmd.spawn((Camera2d, Transform::default()));
    cmd.spawn(GameEvent::None);
    spawn_grid_world(cmd);
}

fn spawn_apple(mut cmd: Commands, server: Res<AssetServer>) {
    log::info!("spawning apple");
    let apple = Apple::new(None);
    cmd.spawn((
        apple,
        Node {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
            left: CELL_SIZE_PX * apple.left() as f32,
            top: CELL_SIZE_PX * apple.top() as f32,
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

fn generate_snake(
    mut cmd: Commands,
    mut apple_query: Query<(Entity, &mut Apple)>,
    mut snake_query: Query<&mut Snake>,
    mut game_event_query: Query<&mut GameEvent>,
    mut snake_painter_query: Query<(Entity, &mut SnakeRenderMarker)>,
    server: Res<AssetServer>,
    time: Res<Time>,
) {
    // check for collisions or paused state
    let game_event = match game_event_query.iter_mut().next() {
        Some(val) => val.clone(),
        None => {
            return;
        }
    };

    match game_event {
        GameEvent::Collision => {
            log::info!("Snake collided with itself");
            return;
        }

        GameEvent::Paused => {
            log::info!("Game is paused");
            return;
        }

        _ => {}
    }

    log::info!("spawning snake");

    let (apple_entity, apple) = apple_query
        .iter_mut()
        .next()
        .expect("expected the apple entity to have already been registered");

    let mut snake = match snake_query.iter_mut().next() {
        Some(mut val) => {
            // timers gotta be ticked, to work
            val.timer.tick(time.delta());
            val
        }

        None => {
            // spawn the snake and return
            let snake = Snake::new(&apple.0);
            for ord in &snake.body {
                cmd.spawn((
                    SnakeRenderMarker,
                    Node {
                        width: CELL_SIZE_PX,
                        height: CELL_SIZE_PX,
                        left: CELL_SIZE_PX * (ord.parent_abs_pos_left as f32),
                        top: CELL_SIZE_PX * (ord.parent_abs_pos_top as f32),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                ));
            }

            cmd.spawn(snake);
            return;
        }
    };

    if !snake.timer.finished() {
        log::info!("Elapsed {:?}", snake.timer.elapsed());
        log::info!("Timer not yet done");
        return;
    }

    // if we're consuming the apple in the move

    match snake.r#move(&apple) {
        GameEvent::EatApple => {
            // despawn apple
            cmd.entity(apple_entity).despawn();

            // spawn new apple
            let apple = Apple::new(Some(&snake));
            cmd.spawn((
                apple,
                Node {
                    width: CELL_SIZE_PX,
                    height: CELL_SIZE_PX,
                    left: CELL_SIZE_PX * apple.left() as f32,
                    top: CELL_SIZE_PX * apple.top() as f32,
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

        GameEvent::Collision => {
            // let's pause the timer and return
            let mut game_event = match game_event_query.iter_mut().next() {
                Some(val) => val,
                None => {
                    return;
                }
            };

            *game_event = GameEvent::Collision;
            snake.timer.pause();
            log::info!("Snake collided with itself");
            return;
        }

        _ => {}
    }

    // despawn current snake
    // snake.reset_timer();
    for (entity, _) in snake_painter_query.iter_mut() {
        cmd.entity(entity).despawn();
    }

    for ord in &snake.body {
        cmd.spawn((
            SnakeRenderMarker,
            Node {
                width: CELL_SIZE_PX,
                height: CELL_SIZE_PX,
                left: CELL_SIZE_PX * (ord.parent_abs_pos_left as f32),
                top: CELL_SIZE_PX * (ord.parent_abs_pos_top as f32),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.0, 0.0, 0.0)),
        ));
    }
}

fn manage_key_input(keys: Res<ButtonInput<KeyCode>>, mut snake_query: Query<&mut Snake>) {
    let mut snake = match snake_query.iter_mut().next() {
        Some(val) => val,
        None => {
            return;
        }
    };

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        if snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }

        return;
    }

    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        if snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }

        return;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        if snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }

        return;
    }

    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        if snake.direction != Direction::Left {
            snake.direction = Direction::Right;
        }

        return;
    }
}

fn spawn_grid_world(mut cmd: Commands) {
    let mut cells = Vec::new();
    for row in 1..=CELL_ROWS {
        for col in 1..=CELL_COLS {
            cells.push((
                Node {
                    width: CELL_SIZE_PX,
                    height: CELL_SIZE_PX,
                    left: CELL_SIZE_PX * ((col - 1) as f32),
                    top: CELL_SIZE_PX * ((row - 1) as f32),
                    border: UiRect::all(BORDER),
                    position_type: PositionType::Absolute,
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(GREEN_COLOR),
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
        .add_systems(Update, (generate_snake, manage_key_input))
        // .add_schedule(schedule)
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
