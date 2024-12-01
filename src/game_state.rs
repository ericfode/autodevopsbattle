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

// Easter egg: "This state machine is powered by pure functional goodness 🎯" 