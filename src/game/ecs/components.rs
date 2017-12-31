use super::specs::{Component, VecStorage};

pub struct Position {
    pub x: i32,
    pub y: i32
}

pub struct Velocity {
    pub x: i32,
    pub y: i32
}

pub struct Display {
    pub rec: super::sdl2::rect::Rect,
    pub color: super::sdl2::pixels::Color
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Component for Display {
    type Storage = VecStorage<Self>;
}