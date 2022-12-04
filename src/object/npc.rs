use crate::render::Renderer;
use crate::menu::{MenuManager, textbox::Textbox};
use crate::object::TObject;

pub enum Character {
    Dad,
    Jodo,
    Sika
}

pub struct NPC {
	pos: (f64, f64),
    pub character: Character
}

impl NPC {
	pub fn new(pos: (f64, f64), character: Character) -> NPC {
		NPC {
            pos, character
        }
	}
}

impl TObject for NPC {
    fn pos(&self) -> (f64, f64) {
        self.pos
    }

    fn update(&self) {

    }

    fn interact(&self, renderer: &mut Renderer, menu_man: &mut MenuManager) -> bool {
        renderer.npc_turn();
        menu_man.open_menu(Box::new(Textbox::new("Hi hungry, I'm dad! Ur a little stinker! A little stinky fellow! A stinky lil guy!".to_string())));
        false
	}
}