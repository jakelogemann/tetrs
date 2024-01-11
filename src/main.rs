//! main application entry point for [`tetris_lib`](../tetris_lib/index.html).
#![allow(non_snake_case)]
#![doc(
  html_no_source,
  html_logo_url = "https://github.com/polis-dev.png?size=128",
  html_favicon_url = "https://github.com/polis-dev.png?size=64",
  issue_tracker_base_url = "https://github.com/polis-dev/rs/issues/"
)]
/// main entry point for the application.
fn main() {
  use bevy::prelude::*;
  App::new().add_plugin(tetris_lib::AppPlugin).run();
}
