use amethyst::ecs::prelude::*;
use crate::constants::*;
use log::*;
use serde_derive::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cardinal {
    North,     // Up
    NorthWest, // Up Left
    NorthEast, // Up Right

    South,     // Down
    SouthWest, // Down Left
    SouthEast, // Down Right

    West, // Left
    East, // Right
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    pub vec: Vec<(usize, f32, f32, f32)>, // index, duration, end_time, rev_end_time
    pub loop_type: AnimationLoop,
    pub timer: f32,
    total_time: f32,
    bounce: bool,
    count: usize,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AnimationLoop {
    Circular,
    Bounce,
    Once,
}

impl Animation {
    pub fn new(vec: Vec<(usize, f32)>, loop_type: AnimationLoop) -> Self {
        let total_time = vec.iter().fold(0.0, |acc, e| acc + e.1);
        let mut acc = 0.0;
        Animation {
            vec: vec
                .iter()
                .map(|(i, d)| {
                    let rev = total_time - acc;
                    acc += d;
                    (*i, *d, acc, rev)
                })
                .collect(),
            loop_type,
            timer: 0.0,
            total_time,
            bounce: false,
            count: 0,
        }
    }

    pub fn with_same_frame_step(
        vec: Vec<usize>,
        loop_type: AnimationLoop,
        frame_step: f32,
    ) -> Self {
        let total_time = vec.len() as f32 * frame_step;
        let mut acc = 0.0;
        Animation {
            vec: vec
                .iter()
                .map(|e| {
                    let rev = total_time - acc;
                    acc += frame_step;
                    (*e, frame_step, acc, rev)
                })
                .collect(),
            loop_type,
            timer: 0.0,
            total_time,
            bounce: false,
            count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
    }

    pub fn update_timer(&mut self, delta: f32) {
        self.timer += delta;
        if self.timer > self.total_time {
            self.timer -= self.total_time;
            self.bounce = !self.bounce;
            self.count += 1;
        }
    }

    pub fn get_frame(&self) -> usize {
        match self.loop_type {
            AnimationLoop::Circular => self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0,
            AnimationLoop::Bounce => {
                if self.bounce {
                    self.vec
                        .iter()
                        .rev()
                        .find(|&&a| self.timer < a.3)
                        .unwrap()
                        .0
                } else {
                    self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0
                }
            }
            AnimationLoop::Once => {
                if self.count > 1 {
                    self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0
                } else {
                    self.vec.iter().last().unwrap().0
                }
            }
        }
    }
}

pub struct Direction {
    pub current: Cardinal,
    pub previous: Option<Cardinal>,
    pub current_anim: String,
}

impl Cardinal {
    pub fn get_x(&self) -> f32 {
        match self {
            Cardinal::West => -1.0,
            Cardinal::NorthWest => -0.7071,
            Cardinal::SouthWest => -0.7071,

            Cardinal::East => 1.0,
            Cardinal::NorthEast => 0.7071,
            Cardinal::SouthEast => 0.7071,

            Cardinal::North => 0.0,
            Cardinal::South => 0.0,
        }
    }

    pub fn get_y(&self) -> f32 {
        match self {
            Cardinal::North => 1.0,
            Cardinal::NorthWest => 0.7071,
            Cardinal::NorthEast => 0.7071,

            Cardinal::South => -1.0,
            Cardinal::SouthWest => -0.7071,
            Cardinal::SouthEast => -0.7071,

            Cardinal::West => 0.0,
            Cardinal::East => 0.0,
        }
    }

    pub fn make_interaction_offset_x(&self) -> f32 {
        match self {
            Cardinal::North => 0.0,
            Cardinal::NorthWest => -BASE / 2.0,
            Cardinal::NorthEast => BASE / 2.0,

            Cardinal::South => 0.0,
            Cardinal::SouthWest => -BASE / 2.0,
            Cardinal::SouthEast => BASE / 2.0,

            Cardinal::West => -BASE / 2.0,
            Cardinal::East => BASE / 2.0,
        }
    }

    pub fn make_interaction_offset_y(&self) -> f32 {
        match self {
            Cardinal::North => BASE / 2.0,
            Cardinal::NorthWest => BASE / 2.0,
            Cardinal::NorthEast => BASE / 2.0,

            Cardinal::South => -BASE / 2.0,
            Cardinal::SouthWest => -BASE / 2.0,
            Cardinal::SouthEast => -BASE / 2.0,

            Cardinal::West => 0.0,
            Cardinal::East => 0.0,
        }
    }
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}
