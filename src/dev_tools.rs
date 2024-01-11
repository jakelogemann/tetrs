//! development tools.
#![allow(unused_imports)]
use crate::prelude::*;
use bevy_inspector_egui::quick::{
  AssetInspectorPlugin, ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
};

/// Developer tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct DevTools;

impl Plugin for DevTools {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Data::default())
      .add_plugin(WorldInspectorPlugin::new().run_if(self::world_inspector_visible))
      .add_system(self::handle_keymaps);
  }
}

fn world_inspector_visible(data: Res<Data>) -> bool {
  data.enabled && data.world
}

fn handle_keymaps(mut data: ResMut<Data>, kb: Res<Input<KeyCode>>) {
  if kb.just_pressed(KeyCode::Backslash) {
    data.enabled = !data.enabled;
  } else if !data.enabled {
    return;
  } else if kb.just_pressed(KeyCode::Minus) {
    data.world = !data.world;
    info!("world inspector: {}", data.world);
  } else if kb.just_pressed(KeyCode::Equals) {
    data.assets = !data.assets;
    info!("assets inspector: {}", data.assets);
  }
}

/// data used by the keymapper
#[derive(Default, Debug, Resource)]
pub struct Data {
  pub enabled: bool,
  pub assets: bool,
  pub resources: bool,
  pub world: bool,
}
