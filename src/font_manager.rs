use sdl2::ttf::{Sdl2TtfContext, Font};

pub struct Fonts<'ttf_module, 'rwops> {
    pub press_start_2p: Font<'ttf_module, 'rwops>,
}

impl<'ttf_module, 'rwops> Fonts<'ttf_module, 'rwops> {
    pub fn load(font_loader: &'ttf_module Sdl2TtfContext) -> Self {
        let press_start_2p = font_loader
            .load_font("assets/PressStart2P-Regular.ttf", 8)
            .unwrap();

        Fonts { press_start_2p }
    }
}

pub struct FontManager<'ttf_module, 'rwops> {
    pub fonts: Fonts<'ttf_module, 'rwops>,
}

impl<'ttf_module, 'rwops> FontManager<'ttf_module, 'rwops> {
    pub fn new(fonts: Fonts<'ttf_module, 'rwops>) -> Self {
        FontManager {
            fonts
        }
    }

    pub fn break_string(&self, str: &String) -> Vec<String> {
		/*
		takes in a string of any length and breaks it into sets of characters at specific length
		the pixel dimensions of a string should be calculable by a function provided by sdl2_ttf
		best guess: starting around ~20 characters, test length, if too long go shorter, if too short go longer
		modify textbox code to take in a Vec<String> instead of just String
		and construct this Vec<String> before the textbox is constructed so it can be passed in
		if we get to the point where we want to actually test this, we need to modify the textbox update to advance through the vec
		this means textbox needs to keep track of the index it is at in the vec, or do pop from vec or something idk

		preferably, we break strings only at whitespace or punctuation, but it will be fine if it doesn't work perfectly yet
		*/
		return Vec::new()
	}
}
