use toxoid_api_macro::component;
use toxoid_api::{IsComponent};
use crate::ecs::*;

component! {
    Position {
        x: u32,
        y: u32,
    },
    Velocity {
        dx: f32,
        dy: f32,
    },
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    },
    Rect {
        width: u32,
        height: u32,
    },
    Color {
        r: u8,
        g: u8,
        b: u8,
    },
    Renderable {
        x: u32,
        y: u32,
    },
    Direction {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    }
}

pub fn init() {
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Rect::register();
    Color::register();
    Renderable::register();
    Direction::register();
}