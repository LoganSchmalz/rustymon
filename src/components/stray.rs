use std::{clone::Clone};

#[derive(Clone)]
pub enum Type {
    //enum for identifying the types of certain moves and strays
    Zen,
    Fire,
    Water,
    Earth,
    Wind,
    Astral,
    Time,
    Dark,
    Light,
}

#[derive(Debug)]
pub enum StrayName {
    //names of all current strays, this code is currently defunct
    Aeternisc,
    Bitaxium,
    Carerus,
    Catis,
    Cespae,
    Omikae,
    Palliub,
    Rubridum,
    Solikigoi,
}

#[derive(Clone)]
pub struct Move {
    //struct defining everything contained within a given move, using the type enum to define the type of the move
    pub name: String,
    pub power: i32,
    pub accuracy: i32,
    pub m_type: Type,
}

#[derive(Clone)]
pub struct Stray {
    //stray data structure, containing mutable list of up to 4 available moves, 
    pub species: String, //species of stray
    pub s_type: Type, //type of species
    pub moves: [Option<Move>; 4], //available moves
    pub hp: i32,  //maximum health points stat
    pub atk: i32, //attack stat
    pub def: i32, //defense stat
    pub spd: i32, //speed stat
    pub cur_hp: i32, //current health points stat
    pub texture: String, //name of the texture file for the given stray
}

impl Default for Stray {
    //default constructor for stray just in case someone forgets to initialize a stray
    fn default() -> Self {
        Self {
            species: String::from("Palliub"),
            s_type: Type::Water,
            moves: [None, None, None, None],
            hp: 15,
            atk: 30,
            def: 20,
            spd: 35,
            cur_hp: 15,
            texture: String::from("assets/strays/palliub.png")
        }
    }
}

impl Stray {
    //constructors for all individual strays in game
    pub fn bitaxium() -> Self {
        Self { //contructor for all of bitaxium's info
            species: String::from("Bitaxium"),
            s_type: Type::Earth,
            moves: [None, None, None, None],
            hp: 20,
            atk: 20,
            def: 40,
            spd: 10,
            cur_hp: 20,
            texture: String::from("assets/strays/bitaxium.png")
        }
    }

    pub fn palliub() -> Self {
        Self { //contructor for all of palliub's info
            species: String::from("Palliub"),
            s_type: Type::Water,
            moves: [None, None, None, None],
            hp: 15,
            atk: 30,
            def: 20,
            spd: 35,
            cur_hp: 15,
            texture: String::from("assets/strays/palliub.png")
        }
    }

    pub fn rubridum() -> Self {
        Self { //contructor for all of rubridum's info
            species: String::from("Rubridum"),
            s_type: Type::Fire,
            moves: [None, None, None, None],
            hp: 15,
            atk: 35,
            def: 25,
            spd: 25,
            cur_hp: 15,
            texture: String::from("assets/strays/rubridum.png")
        }
    }

    pub fn aeternisc() -> Self {
        Self { //contructor for all of aeternisc's info
            species: String::from("Aeternisc"),
            s_type: Type::Time,
            moves: [None, None, None, None],
            hp: 50,
            atk: 25,
            def: 35,
            spd: 10,
            cur_hp: 50,
            texture: String::from("assets/strays/aeternisc.png")
        }
    }

    pub fn solikigoi() -> Self {
        Self { //contructor for all of solikigoi's info
            species: String::from("Solikigoi"),
            s_type: Type::Water,
            moves: [None, None, None, None],
            hp: 20,
            atk: 15,
            def: 35,
            spd: 15,
            cur_hp: 20,
            texture: String::from("assets/strays/solikigoi.png")
        }
    }

    pub fn catis() -> Self {
        Self { //contructor for all of catis's info
            species: String::from("Catis"),
            s_type: Type::Earth,
            moves: [None, None, None, None],
            hp: 25,
            atk: 15,
            def: 30,
            spd: 10,
            cur_hp: 25,
            texture: String::from("assets/strays/catis.png")
        }
    }
    
    pub fn cespae() -> Self {
        Self { //constructor for all of cespae's info 
            species: String::from("Cespae"),
            s_type: Type::Zen,
            moves: [None, None, None, None],
            hp: 15,
            atk: 20,
            def: 30,
            spd: 20,
            cur_hp: 15,
            texture: String::from("assets/strays/cespae.png")
        }
    }

    pub fn omikae() -> Self {
        Self { //constructor for all of omikae's info 
            species: String::from("Omikae"),
            s_type: Type::Astral,
            moves: [None, None, None, None],
            hp: 20,
            atk: 15,
            def: 30,
            spd: 20,
            cur_hp: 20,
            texture: String::from("assets/strays/omikae.png")
        }
    }

    pub fn carerus() -> Self {
        Self { //constructor for all of carerus's info 
            species: String::from("Carerus"),
            s_type: Type::Dark,
            moves: [None, None, None, None],
            hp: 20,
            atk: 30,
            def: 15,
            spd: 20,
            cur_hp: 20,
            texture: String::from("assets/strays/carerus.png")
        }
    }
}


