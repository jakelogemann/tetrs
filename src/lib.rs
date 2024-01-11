//! library for [`tetris`](../tetris/index.html).
#![doc(
  html_no_source,
  html_logo_url = "https://github.com/polis-dev.png?size=128",
  html_favicon_url = "https://github.com/polis-dev.png?size=64",
  issue_tracker_base_url = "https://github.com/polis-dev/rs/issues/"
)]
pub mod assets;
pub mod board;
pub mod dev_tools;
pub mod keymap;
pub mod menu;
pub mod palette;
pub mod score;
pub mod state;
pub mod tetromino;
pub mod timers;

pub(crate) mod prelude;
use crate::prelude::*;

/// name of this game as a static string.
pub const GAME_NAME: &str = "Tetris";

/// the overall application plugin.
pub struct AppPlugin;

impl Plugin for AppPlugin {
  fn build(&self, app: &mut App) {
    Self::initialize_plugin(app);
    Self::diagnostics(app);
    Self::register_types(app);
    Self::register_states(app);
    // core application logic.
    app
      .init_resource::<timers::GameTimers>()
      .insert_resource({
        let mut queue = VecDeque::new();
        queue.extend(Tetromino::random_n(7));
        Queue(queue)
      })
      .add_plugin(GameBoard);
    // global keymaps & app management.
    Self::add_systems(app);
  }
}

impl AppPlugin {
  /// diagnostics.
  fn diagnostics(_app: &mut App) {
    let build_info = get_build_info().to_owned();
    let vcs_info = build_info.version_control.as_ref().expect("we use a vcs");
    let git_info = vcs_info.git().expect("we use git as our vcs");
    info!(
      "{} {} ({})",
      GAME_NAME,
      build_info.crate_info.version.to_string(),
      build_info.profile
    );
    info!("{:?}", git_info);
    info!("Build Time: {:?}", build_info.timestamp);
    info!("{:?}", build_info.compiler);
    info!("{:?}", build_info.target);
  }

  /// minimal initialization of the plugin.
  fn initialize_plugin(app: &mut App) {
    app
      .add_plugins(
        DefaultPlugins
          .set(WindowPlugin {
            close_when_requested: true,
            primary_window: Some(Window {
              title: crate::GAME_NAME.into(),
              present_mode: bevy::window::PresentMode::AutoVsync,
              focused: true,
              // Tells wasm to resize the window according to the available canvas
              fit_canvas_to_parent: true,
              // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
              prevent_default_event_handling: false,
              resizable: false,
              ..default()
            }),
            ..default()
          })
          .set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions::with_num_threads(4),
          }),
      )
      .insert_resource(ClearColor(palette::BACKGROUND))
      .add_plugin(bevy_egui::EguiPlugin)
      .add_plugin(DevTools)
      .init_resource::<GameAssets>();
  }

  /// register all types used by this plugin.
  fn register_types(app: &mut App) {
    app
      .add_event::<GameEvent>()
      .register_type::<GameTimers>()
      .register_type::<GameAssets>()
      .register_type::<GameState>()
      .register_type::<AppState>();
  }

  /// register all states used by this plugin.
  fn register_states(app: &mut App) {
    app.add_state::<AppState>().add_state::<GameState>();
  }

  /// setup egui with our custom theme.
  fn setup_egui(mut _contexts: EguiContexts) {
    // catppuccin_egui::set_theme(&contexts.ctx_mut(), catppuccin_egui::MOCHA);
  }

  /// spawn the main camera entity.
  fn spawn_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default())
      .insert(Name::new("Main Camera"));
  }

  /// add all systems used by this plugin.
  fn add_systems(app: &mut App) {
    app
    .add_startup_systems((Scoreboard::setup, Self::setup_egui, Self::spawn_camera))
    // game logic
    .add_systems((
      keymap::handle_quit,
      keymap::handle_game_events,
      keymap::emit_game_events,
      bevy::window::close_when_requested,
      crate::state::pause_game.run_if(
          state_exists_and_equals(GameState::Playing)
              .or_else(state_exists_and_equals(GameState::Paused)),
      ),
      // Game Restarted
      Score::reset.in_schedule(OnEnter(GameState::Restarting)),
      crate::state::play_game.in_set(OnUpdate(GameState::Restarting)),
      // menu setup systems:
      crate::menu::setup_game_paused_menu.in_schedule(OnEnter(GameState::Paused)),
      crate::menu::setup_game_over_menu.in_schedule(OnEnter(AppState::GameOver)),
      // menu cleanup systems:
      despawn_screen::<crate::menu::OnGamePausedMenuScreen>.in_schedule(OnExit(GameState::Paused)),
      despawn_screen::<crate::menu::OnMainMenuScreen>.in_schedule(OnExit(AppState::MainMenu)),
      // menu button system.
      crate::menu::click_button.run_if(
          state_exists_and_equals(AppState::MainMenu)
              .or_else(state_exists_and_equals(AppState::GameOver))
              .or_else(state_exists_and_equals(GameState::Paused)),
      ),
    ))
    // Game Playing
    .add_systems(
        (
            Movable::update_pieces,
            board::remove_piece_component,
            board::detect_game_over.after(board::remove_piece_component),
            board::check_full_line.after(board::remove_piece_component),
        )
            .in_base_set(CoreSet::PostUpdate)
            .distributive_run_if(GameState::is_playing_state),
    )
    .add_systems(
        (
            NextTetromino::update,
            tetromino::spawn_next_piece,
            GameBoard::update_tile_visibility,
            keymap::rotate_piece,
            keymap::move_piece,
            keymap::hold_piece,
            Scoreboard::update,
        )
            .in_set(OnUpdate(GameState::Playing)),
    )
    // Main Menu
    .add_systems(
        (
            crate::menu::setup_main_menu,
            Tile::clear_all,
            Score::reset,
            NextTetromino::clear,
        )
            .in_schedule(OnEnter(AppState::MainMenu)),
    )
    // Game Over Menu
    .add_systems(
        (
            despawn_screen::<crate::menu::OnGameOverMenuScreen>,
            Tile::clear_all,
            Score::reset,
            NextTetromino::clear,
        )
            .in_schedule(OnExit(AppState::GameOver)),
    );
  }
}
