use bevy::ecs::component::Component;

#[derive(Clone, Copy, Debug, Component)]
pub enum GameEvent {
    None,
    EatApple,
    Collision,
    Paused,
}
