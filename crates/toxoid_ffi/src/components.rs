use toxoid_api_macro::component;
use toxoid_api::{IsComponent};
use crate::ecs::*;

pub enum DirectionEnum {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

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
        direction: u8
    },
    // TODO: Make these all tags with no fields and create tag!{} proc macro
    // Also move to snake_engine
    Player {
        head: bool
    },
    Food {
        food: bool
    },
    Despawn {
        despawn: bool
    }
    // Head {
    //     head: bool
    // },
    // Spawned {
    //     spawned: bool
    // },  
}

pub fn init() {
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Rect::register();
    Color::register();
    Renderable::register();
    Direction::register();
    Player::register();
    Food::register();
}