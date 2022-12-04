use sdl2::{video::Window, render::Canvas, rect::Rect};

use crate::texture_manager::{TextureManager};
use crate::font_manager::{FontManager};
use crate::render::{PIXELS_X, PIXELS_Y};

use super::{MenuItem, Action};

#[derive(PartialEq)]
pub enum MainMenuButton {
	StartButton,
    LoadButton,
    SettingsButton,
}

pub struct MainMenu {
	curr_button: MainMenuButton
}

impl MainMenu {
	pub fn new() -> MainMenu {
		MainMenu {
			curr_button: StartButton
		}
	}
}

use MainMenuButton::*;

impl MenuItem for MainMenu {
	fn render(&mut self, canvas: &mut Canvas<Window>, texture_manager: &mut TextureManager, _font_man: &FontManager) {
		if self.curr_button == StartButton {
            texture_manager.textures.start_button.set_color_mod(223, 3, 67);
        } else {
            texture_manager.textures.start_button.set_color_mod(255, 255, 255);
        }

        if self.curr_button == LoadButton {
            texture_manager.textures.load_button.set_color_mod(223, 3, 67);
        } else {
            texture_manager.textures.load_button.set_color_mod(255, 255, 255);
        }

        if self.curr_button == SettingsButton {
			texture_manager.textures.settings_button.set_color_mod(223, 3, 67);
        } else {
            texture_manager.textures.settings_button.set_color_mod(255, 255, 255);
        }
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        let start_quad = Rect::new(82, 100, 75, 24);
        let load_quad = Rect::new(102, 122, 16, 16);
        let settings_quad = Rect::new(121, 122, 16, 16);

        canvas.copy(&texture_manager.textures.main_menu, None, screen_quad).unwrap();
        canvas
            .copy(&texture_manager.textures.start_button, None, start_quad)
            .unwrap();
        canvas.copy(&texture_manager.textures.load_button, None, load_quad).unwrap();
        canvas
            .copy(&texture_manager.textures.settings_button, None, settings_quad)
            .unwrap();
	}

	fn update(&mut self, action: Action) -> bool {
		match action {
			Action::ACCEPT => {
				if self.curr_button == StartButton {
					return true;
				}
			}
			Action::LEFT => {
				self.curr_button = match self.curr_button {
					StartButton => LoadButton,
					LoadButton => StartButton,
					SettingsButton => LoadButton,
				}
			}
			Action::RIGHT => {
				self.curr_button = match self.curr_button {
					StartButton => SettingsButton, //BUTTONS[2] == Button::SettingsButton
					LoadButton => SettingsButton, //BUTTONS[2] == Button::SettingsButton
					SettingsButton => StartButton, //BUTTONS[0] == Button::StartButton
				}
			}
			Action::UP => {
				self.curr_button = match self.curr_button {
					StartButton => SettingsButton,
					SettingsButton => StartButton,
					LoadButton => StartButton
				}
			}
			Action::DOWN => {
				self.curr_button = match self.curr_button {
					StartButton => LoadButton,
					SettingsButton => StartButton,
					LoadButton => SettingsButton
				}
			}
			_ => {}
		}
		false
	}
}