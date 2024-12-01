use bevy::prelude::*;

// Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Planning,
    Running,
    Paused,
}

pub mod components;
pub mod resources;
pub mod systems;
pub mod plugins;

#[cfg(test)]
pub mod test_utils;

// Easter egg: "This module tree is well-balanced, just like all things should be 🌳" 