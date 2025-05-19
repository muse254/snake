use bevy::ecs::component::Component;
use rand::Rng;

use crate::{
    magic_numbers::{CELL_COLS, CELL_ROWS},
    snake::{Ordinance, Snake},
};

#[derive(Component, Clone, Copy)]
pub struct Apple(pub Ordinance);

impl Apple {
    pub fn new(snake: Option<&Snake>) -> Self {
        let snake_positions = match snake {
            Some(snake) => &snake.body,
            None => &Vec::new(),
        };

        let mut rng = rand::rng();
        let mut apple_pos: Ordinance;
        loop {
            let top = rng.random_range(0..CELL_ROWS);
            let left = rng.random_range(0..CELL_COLS);

            apple_pos = Ordinance {
                parent_abs_pos_left: left,
                parent_abs_pos_top: top,
            };

            if snake_positions
                .iter()
                .any(|pos| pos.parent_abs_pos_left == apple_pos.parent_abs_pos_left)
                && snake_positions
                    .iter()
                    .any(|pos| pos.parent_abs_pos_top == apple_pos.parent_abs_pos_top)
            {
                // If the apple is spawned on the snake, spawn it again
                continue;
            }
            break;
        }

        Apple(apple_pos)
    }

    pub fn left(&self) -> i8 {
        self.0.parent_abs_pos_left
    }

    pub fn top(&self) -> i8 {
        self.0.parent_abs_pos_top
    }
}
