//! color palette.
#![allow(unused)]
use bevy::prelude::Color;

// Color Palette (catppuccin machiato)
// ===================================
/// Rosewater 	#f4dbd6 	rgb(244, 219, 214) 	hsl(10, 58%, 90%)
pub const ROSEWATER: Color = Color::rgb(0.96, 0.86, 0.84);
/// Flamingo 	#f0c6c6 	rgb(240, 198, 198) 	hsl(0, 58%, 86%)
pub const FLAMINGO: Color = Color::rgb(0.94, 0.78, 0.78);
/// Pink 	#f5bde6 	rgb(245, 189, 230) 	hsl(316, 74%, 85%)
pub const PINK: Color = Color::rgb(0.96, 0.74, 0.90);
/// Mauve 	#c6a0f6 	rgb(198, 160, 246) 	hsl(267, 83%, 80%)
pub const MAUVE: Color = Color::rgb(0.78, 0.63, 0.96);
/// Red 	#ed8796 	rgb(237, 135, 150) 	hsl(351, 74%, 73%)
pub const RED: Color = Color::rgb(0.93, 0.53, 0.59);
/// Maroon 	#ee99a0 	rgb(238, 153, 160) 	hsl(355, 71%, 77%)
pub const MAROON: Color = Color::rgb(0.93, 0.60, 0.63);
/// Peach 	#f5a97f 	rgb(245, 169, 127) 	hsl(21, 86%, 73%)
pub const PEACH: Color = Color::rgb(0.96, 0.66, 0.50);
/// Yellow 	#eed49f 	rgb(238, 212, 159) 	hsl(40, 70%, 78%)
pub const YELLOW: Color = Color::rgb(0.93, 0.83, 0.62);
/// Green 	#a6da95 	rgb(166, 218, 149) 	hsl(105, 48%, 72%)
pub const GREEN: Color = Color::rgb(0.65, 0.86, 0.58);
/// Teal 	#8bd5ca 	rgb(139, 213, 202) 	hsl(171, 47%, 69%)
pub const TEAL: Color = Color::rgb(0.55, 0.84, 0.79);
/// Sky 	#91d7e3 	rgb(145, 215, 227) 	hsl(189, 59%, 73%)
pub const SKY: Color = Color::rgb(0.57, 0.84, 0.89);
/// Sapphire 	#7dc4e4 	rgb(125, 196, 228) 	hsl(199, 66%, 69%)
pub const SAPPHIRE: Color = Color::rgb(0.49, 0.77, 0.89);
/// Blue 	#8aadf4 	rgb(138, 173, 244) 	hsl(220, 83%, 75%)
pub const BLUE: Color = Color::rgb(0.54, 0.68, 0.96);
/// Lavender 	#b7bdf8 	rgb(183, 189, 248) 	hsl(234, 82%, 85%)
pub const LAVENDER: Color = Color::rgb(0.72, 0.74, 0.97);
/// Text 	#cad3f5 	rgb(202, 211, 245) 	hsl(227, 68%, 88%)
pub const TEXT: Color = Color::rgb(0.79, 0.83, 0.96);
/// Subtext1 	#b8c0e0 	rgb(184, 192, 224) 	hsl(228, 39%, 80%)
pub const SUBTEXT1: Color = Color::rgb(0.72, 0.75, 0.88);
/// Subtext0 	#a5adcb 	rgb(165, 173, 203) 	hsl(227, 27%, 72%)
pub const SUBTEXT0: Color = Color::rgb(0.65, 0.68, 0.80);
/// Overlay2 	#939ab7 	rgb(147, 154, 183) 	hsl(228, 20%, 65%)
pub const OVERLAY2: Color = Color::rgb(0.58, 0.60, 0.72);
/// Overlay1 	#8087a2 	rgb(128, 135, 162) 	hsl(228, 15%, 57%)
pub const OVERLAY1: Color = Color::rgb(0.50, 0.53, 0.63);
/// Overlay0 	#6e738d 	rgb(110, 115, 141) 	hsl(230, 12%, 49%)
pub const OVERLAY0: Color = Color::rgb(0.43, 0.45, 0.55);
/// Surface2 	#5b6078 	rgb(91, 96, 120) 	hsl(230, 14%, 41%)
pub const SURFACE2: Color = Color::rgb(0.36, 0.38, 0.47);
/// Surface1 	#494d64 	rgb(73, 77, 100) 	hsl(231, 16%, 34%)
pub const SURFACE1: Color = Color::rgb(0.29, 0.30, 0.39);
/// Surface0 	#363a4f 	rgb(54, 58, 79) 	hsl(230, 19%, 26%)
pub const SURFACE0: Color = Color::rgb(0.21, 0.23, 0.30);
/// Base 	#24273a 	rgb(36, 39, 58) 	hsl(232, 23%, 18%)
pub const BASE: Color = Color::rgb(0.14, 0.15, 0.23);
/// Mantle 	#1e2030 	rgb(30, 32, 48) 	hsl(233, 23%, 15%)
pub const MANTLE: Color = Color::rgb(0.12, 0.13, 0.19);
/// Crust 	#181926 	rgb(24, 25, 38) 	hsl(236, 23%, 12%)
pub const CRUST: Color = Color::rgb(0.09, 0.10, 0.15);

// Color Names
// ===========
/// Background color, used for the window's background.
pub const BACKGROUND: Color = BASE;
/// Score color, used for the scoreboard text.
pub const SCORE: Color = SKY;
/// Paddle color, used for the paddle(s).
pub const PADDLE: Color = BLUE;
/// Wall color, used for the wall around the arena.
pub const WALL: Color = MANTLE;
/// Ball color, used for the ball.
pub const BALL: Color = MAUVE;
/// Normal button color.
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/// Normal button color, when hovered with mouse.
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
/// Normal button color, when hovered & pressed.
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
/// Normal button color, pressed.
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
/// Background color, used for the menu's background.
pub const MENU_BACKGROUND: Color = CRUST;
/// possible background colors for the bricks.
pub const BRICKS: [Color; 12] = [
  BLUE, FLAMINGO, GREEN, LAVENDER, MAROON, MAUVE, PEACH, PINK, RED, ROSEWATER, SAPPHIRE, TEAL,
];
