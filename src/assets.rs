//! game-specific assets.
use bevy::prelude::*;

/// resource containing references to game-specific audio resources.
#[derive(Debug, Resource, Reflect)]
pub struct GameAssets {
  /// sound effect for when a piece is dropped.
  drop_sound: Handle<AudioSource>,
  /// sound effect for when the game is over.
  gameover_sound: Handle<AudioSource>,
  /// sound effect for when a line is cleared.
  line_clear_sound: Handle<AudioSource>,
  /// bold font for the game.
  main_bold_font: Handle<Font>,
  /// icon for the game.
  game_icon: Handle<Image>,
}

impl GameAssets {
  /// play the drop sound effect.
  pub fn play_drop(&self, audio: &Res<Audio>) {
    audio.play(self.drop_sound.clone());
  }
  /// play the line clear sound effect.
  pub fn play_line_clear(&self, audio: &Res<Audio>) {
    audio.play(self.line_clear_sound.clone());
  }
  /// play the game over sound effect.
  pub fn play_gameover(&self, audio: &Res<Audio>) {
    audio.play(self.gameover_sound.clone());
  }
  /// return a new [`Handle`] to the bold font.
  pub fn bold_font(&self) -> Handle<Font> {
    self.main_bold_font.clone()
  }
  /// return a new [`UiImage`] with the game icon.
  pub fn game_icon(&self) -> UiImage {
    UiImage::new(self.game_icon.clone())
  }

  /// return the font size for a default button
  pub fn button_font_size(&self) -> f32 {
    20.0
  }
  pub fn button_text_style(&self) -> TextStyle {
    TextStyle {
      font: self.bold_font(),
      font_size: self.button_font_size(),
      color: Color::rgb(0.9, 0.9, 0.9),
    }
  }

  pub fn button_text_bundle(&self, text: &str) -> TextBundle {
    TextBundle::from_section(text, self.button_text_style())
  }
}

impl FromWorld for GameAssets {
  fn from_world(world: &mut World) -> Self {
    let assets = world
      .get_resource_mut::<AssetServer>()
      .expect("AssetServer is always initialized.");
    Self {
      drop_sound: assets.load("sounds/Drop.wav"),
      gameover_sound: assets.load("sounds/Gameover.wav"),
      line_clear_sound: assets.load("sounds/Lineclear.wav"),
      main_bold_font: assets.load("fonts/FiraSans-Bold.ttf"),
      game_icon: assets.load("icon.png"),
    }
  }
}
