use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::plugins::UiPlugin;
use resources::{GameResources, create_initial_system};
use components::SystemGraph;
use systems::game_loop::tick_system;

mod components;
mod resources;
mod systems;
mod plugins;

// Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Planning,
    Execution,
    GameOver,
}

// Phase tracking for execution state
#[derive(Resource)]
struct ExecutionPhase {
    current_step: usize,
    steps: Vec<String>,
    elapsed_time: f64,
}

fn main() {
    App::new()
        // Add core Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DevOps Entropy".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                mode: bevy::window::WindowMode::Windowed,
                position: bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // Add egui for technical UI
        .add_plugins(EguiPlugin)
        .add_plugins(UiPlugin)
        
        // Add game states
        .add_state::<GameState>()
        
        // Add core resources
        .insert_resource(ExecutionPhase {
            current_step: 0,
            steps: vec![
                "Apply Changes".into(),
                "Run Simulation".into(),
                "Process Defects".into(),
                "Calculate Revenue".into(),
                "Update Tech Debt".into(),
            ],
            elapsed_time: 0.0,
        })
        .insert_resource(GameResources::default())
        
        // Add startup system to initialize game
        .add_systems(Startup, setup_game)
        
        // Systems that run in specific states
        .add_systems(OnEnter(GameState::Planning), setup_planning_phase)
        .add_systems(OnEnter(GameState::Execution), setup_execution_phase)
        .add_systems(Update, 
            (
                update_planning_phase.run_if(in_state(GameState::Planning)),
                (update_execution_phase, tick_system).run_if(in_state(GameState::Execution)),
            )
        )
        .run();
}

// Initialize game state and spawn initial system
fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Setting up initial game state");
    
    // Create initial system graph
    let system = create_initial_system();
    info!("Created initial system with {} nodes and {} edges", 
        system.graph.node_count(), 
        system.graph.edge_count()
    );
    
    commands.spawn(system);
    
    // Set initial state to Planning
    next_state.set(GameState::Planning);
    info!("Set game state to Planning");
}

// Remove old setup_planning_phase since we don't want to respawn the system
fn setup_planning_phase() {
    // Planning phase is set up in setup_game
}

fn setup_execution_phase(mut commands: Commands) {
    info!("Starting execution phase");
}

fn update_planning_phase(
    mut next_state: ResMut<NextState<GameState>>,
    resources: Res<GameResources>,
    query: Query<&SystemGraph>,
) {
    // Debug info
    if let Ok(system) = query.get_single() {
        info!(
            "System Status - Nodes: {}, Edges: {}, Money: ${:.2}, Sprint: {}",
            system.graph.node_count(),
            system.graph.edge_count(),
            resources.money,
            resources.sprint
        );
    }
    
    // TODO: Add a way to transition to execution phase (e.g., button press)
    // For now, let's automatically transition after 5 seconds
    if resources.sprint == 1 {  // Only in first sprint for testing
        next_state.set(GameState::Execution);
    }
}

fn update_execution_phase(
    mut next_state: ResMut<NextState<GameState>>,
    mut execution_phase: ResMut<ExecutionPhase>,
    time: Res<Time>,
    resources: Res<GameResources>,
) {
    // Update elapsed time
    execution_phase.elapsed_time += time.delta_seconds_f64();
    
    // Print debug info every second
    if execution_phase.elapsed_time.floor() > (execution_phase.elapsed_time - time.delta_seconds_f64()).floor() {
        info!(
            "Execution Phase - Step: {}/{}, Money: ${:.2}, Time: {:.1}s",
            execution_phase.current_step + 1,
            execution_phase.steps.len(),
            resources.money,
            execution_phase.elapsed_time
        );
    }
    
    // After 10 seconds, go back to planning
    if execution_phase.elapsed_time >= 10.0 {
        execution_phase.elapsed_time = 0.0;
        next_state.set(GameState::Planning);
        info!("Execution phase complete, returning to planning");
    }
}

// Easter egg: Hidden developer commentary
#[allow(dead_code)]
const DEVELOPER_NOTES: &str = "
    🦀 Dear future maintainer,
    
    I hope you're having a wonderful day. Yes, this game is about managing technical debt,
    and yes, we've tried our best to keep this codebase clean. If you find any tech debt here,
    consider it a meta-commentary on the nature of software development.
    
    Remember: In theory, there is no difference between theory and practice.
    In practice, there is. But in Rust, at least the compiler has your back!
    
    P.S. The coffee machine microservice is actually running in production.
    Don't ask how. Don't ask why. Just keep it running.
";
