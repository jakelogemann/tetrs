//! re-export of [`bevy::prelude`] with our customizations.
#[doc(inline)]
pub(crate) use {
  crate::{
    assets::GameAssets,
    board::{GameBoard, GameBoardData},
    dev_tools::DevTools,
    keymap::GameEvent,
    palette,
    score::{Score, Scoreboard},
    state::{AppState, GameState},
    tetromino::{Movable, Queue, Shape, Tetromino, Tile, NextTetromino},
    timers::GameTimers,
  },
  bevy::{app::AppExit, prelude::*},
  bevy_inspector_egui::bevy_egui::{self, EguiContexts},
  rand::Rng,
  std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fmt,
    ops::Add,
    time::Duration,
  },
};

/// helper function to despawn all entities with a given component.
pub(crate) fn despawn_screen<T: Component>(
  to_despawn: Query<Entity, With<T>>,
  mut commands: Commands,
) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

build_info::build_info!(pub fn get_build_info);
