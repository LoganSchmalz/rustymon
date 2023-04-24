/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for organizing and rendering text
/****************************************************/
use regex::Regex;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Fonts<'ttf_module, 'rwops> {
    pub munro: Font<'ttf_module, 'rwops>, //munro font
}

impl<'ttf_module, 'rwops> Fonts<'ttf_module, 'rwops> {
    //function for loading an individual font
    //takes in an sdl2 font context to load the font and returns whether or not the font could be loaded
    pub fn load(font_loader: &'ttf_module Sdl2TtfContext) -> Result<Self, String> {
        let munro = font_loader.load_font("assets/fonts/munro-small.ttf", 10)?;

        Ok(Fonts { munro })
    }
}

pub struct FontManager<'ttf_module, 'rwops> {
    pub fonts: Fonts<'ttf_module, 'rwops>, //stores fonts
}

impl<'ttf_module, 'rwops> FontManager<'ttf_module, 'rwops> {
    //constructs new FontManager object from a given font object, which is just a list of (currently one) font(s)
    pub fn new(fonts: Fonts<'ttf_module, 'rwops>) -> Self {
        FontManager { fonts }
    }
    
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
    pub fn break_string(&self, str: &str, box_w: u32) -> Vec<String> {
        
        let punc = Regex::new("[.?\n \t!,:;]").unwrap();

        let mut ret: Vec<String> = vec![];
        //let pad_y = 10 as u32;w
        let pad_x = 10u32;

        let line_w = box_w - pad_x * 2;
        println!("linew:{}", line_w);

        let mut curr_str: String = "".to_string();
        let mut next_word: String = "".to_string();
        let mut buff = [0u8; 4];
        let mut curr_char: &str;
        let mut cs_len: u32;
        let mut nw_len: u32;

        for (_i, c) in str.chars().enumerate() {
            next_word.push(c);

            curr_char = c.encode_utf8(&mut buff);

            if punc.is_match(curr_char) {
                cs_len = self
                    .fonts
                    .munro
                    .size_of(curr_str.as_str())
                    .ok()
                    .unwrap()
                    .0;
                nw_len = self
                    .fonts
                    .munro
                    .size_of(next_word.as_str())
                    .ok()
                    .unwrap()
                    .0;

                if (cs_len + nw_len) < line_w {
                    curr_str += next_word.as_str();
                } else {
                    ret.push(curr_str);
                    curr_str = next_word;
                }

                next_word = "".to_string();
            }
        }

        if curr_str != *"" || next_word != *"" {
            curr_str += next_word.as_str();
            ret.push(curr_str);
        }

        for i in &ret {
            let i: &String = i;
            println!("v{}", i);
        }

        ret
    }
}
