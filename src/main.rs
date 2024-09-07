use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use leafwing_input_manager::prelude::*;
use tiny_bail::prelude::*;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use iyes_progress::{ProgressCounter, ProgressPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins((
            ProgressPlugin::new(GameState::AssetLoading).continue_to(GameState::Loaded),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(DefaultPickingPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).load_collection::<GameAssets>(),
        )
        .add_systems(Startup, setup_actions)
        .add_systems(
            Update,
            (print_progress)
                .chain()
                .run_if(in_state(GameState::AssetLoading))
                .after(LoadingStateSet(GameState::AssetLoading)),
        )
        .add_systems(
            Update,
            (quit)
                .chain()
                .run_if(in_state(GameState::Loaded))
                .after(LoadingStateSet(GameState::Loaded)),
        )
        .add_systems(OnEnter(GameState::Quit), on_quit)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Quit,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Loaded,
    Quit,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {}

fn setup_actions(mut commands: Commands) {
    let input_map = InputMap::new([(Action::Quit, KeyCode::KeyQ)]);
    commands.spawn(InputManagerBundle::with_map(input_map));
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}

fn quit(query: Query<&ActionState<Action>>, mut next_state: ResMut<NextState<GameState>>) {
    let action_state = query.single();
    if action_state.just_pressed(&Action::Quit) {
        next_state.set(GameState::Quit);
    }
}

fn on_quit(mut quit: EventWriter<AppExit>) {
    info!("Quitting...");
    quit.send(AppExit::Success);
}
