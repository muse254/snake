use bevy::{ecs::component::Component, time::Timer};
use rand::Rng;

use crate::magic_numbers::{CELL_COLS, CELL_ROWS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 1,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(1..=4) {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ordinance {
    pub parent_abs_pos_left: i8,
    pub parent_abs_pos_top: i8,
}

impl Ordinance {
    fn get_left(&self) -> Ordinance {
        // offset to the right of the screen if operation is negative
        let mut left = self.parent_abs_pos_left - 1;
        if left < 0 {
            left += CELL_COLS;
        }

        Ordinance {
            parent_abs_pos_left: left,
            parent_abs_pos_top: self.parent_abs_pos_top,
        }
    }

    fn get_top(&self) -> Ordinance {
        // offset to the bottom of the screen if operation is negative
        let mut top = self.parent_abs_pos_top - 1;
        if top < 0 {
            top += CELL_ROWS;
        }

        Ordinance {
            parent_abs_pos_left: self.parent_abs_pos_left,
            parent_abs_pos_top: top,
        }
    }

    fn get_right(&self) -> Ordinance {
        // offset to the left of the screen if operation is greater than the max column
        let mut right = self.parent_abs_pos_left + 1;
        if right >= CELL_COLS {
            right -= CELL_COLS;
        }

        Ordinance {
            parent_abs_pos_left: right,
            parent_abs_pos_top: self.parent_abs_pos_top,
        }
    }

    fn get_bottom(&self) -> Ordinance {
        // offset to the top of the screen if operation is greater than the max row
        let mut bottom = self.parent_abs_pos_top + 1;
        if bottom >= CELL_ROWS {
            bottom -= CELL_ROWS;
        }

        Ordinance {
            parent_abs_pos_left: self.parent_abs_pos_left,
            parent_abs_pos_top: bottom,
        }
    }
}

#[derive(Component)]
pub struct SnakeRenderMarker;

#[derive(Debug, Component)]
pub struct Snake {
    pub body: Vec<Ordinance>,
    pub direction: Direction,
    pub timer: Timer,
}

impl Default for Snake {
    fn default() -> Self {
        let mut rng = rand::rng();
        let mut body = vec![Ordinance {
            parent_abs_pos_left: rng.random_range(0..CELL_COLS),
            parent_abs_pos_top: rng.random_range(0..CELL_ROWS),
        }];

        let direction = Direction::default();
        match direction {
            Direction::Up => {
                let abdomen = body[0].get_bottom();
                let tail = abdomen.get_bottom();
                body.push(abdomen);
                body.push(tail);
            }

            Direction::Down => {
                let abdomen = body[0].get_top();
                let tail = abdomen.get_top();
                body.push(abdomen);
                body.push(tail);
            }

            Direction::Left => {
                let abdomen = body[0].get_right();
                let tail = abdomen.get_right();
                body.push(abdomen);
                body.push(tail);
            }

            Direction::Right => {
                let abdomen = body[0].get_left();
                let tail = abdomen.get_left();
                body.push(abdomen);
                body.push(tail);
            }
        }

        Self {
            body,
            direction,
            timer: Timer::from_seconds(0.5, bevy::time::TimerMode::Once),
        }
    }
}

impl Snake {
    pub fn new(apple_spawned: &Ordinance) -> Self {
        let mut snake = Snake::default();

        // if any part of the snake is on the apple, respawn
        while snake.body.iter().any(|ord| {
            ord.parent_abs_pos_left
                .eq(&apple_spawned.parent_abs_pos_left)
                && ord.parent_abs_pos_top.eq(&apple_spawned.parent_abs_pos_top)
        }) {
            snake = Snake::default();
        }

        snake
    }

    pub fn score(&self) -> u32 {
        (self.body.len() - 1) as u32
    }

    pub fn reset_timer(&mut self) {
        self.timer = Timer::from_seconds(0.5, bevy::time::TimerMode::Once);
    }

    pub fn r#move(&mut self, direction: Option<Direction>) {
        // if we are provided with a move
        match direction {
            // Some(Direction::Up) => {
            //     // checking
            // }
            // Some(Direction::Down) => {}
            // Some(Direction::Left) => {}
            // Some(Direction::Right) => {}
            None => {
                // remove the tail
                self.body.pop();

                // advance the head in direction
                let mut new_body = match self.direction {
                    Direction::Up => {
                        vec![self.body[0].get_top()]
                    }
                    Direction::Down => {
                        vec![self.body[0].get_bottom()]
                    }
                    Direction::Left => {
                        vec![self.body[0].get_left()]
                    }
                    Direction::Right => {
                        vec![self.body[0].get_right()]
                    }
                };

                new_body.append(&mut self.body);
                self.body = new_body;
            }

            _ => {
                unimplemented!()
            }
        }

        self.reset_timer();
    }
}
