use super::specs::{Component, VecStorage};

pub struct Position {
    pub x: i32,
    pub y: i32
}

pub struct Velocity {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}