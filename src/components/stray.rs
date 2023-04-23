/****************************************************/
// Created by: Tucker McCulloch
// Description: Data structures for strays and related information including types and moves
// also contains defintions for all individual strays, moves, types, and kinds of moves
/****************************************************/

use std::{clone::Clone};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MoveKind {
    //enum listing the kinds of moves (could later be expanded to include things like DOT, different kinds of AOE, etc.)
    Healing,
    Damage,
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
pub struct Move {
    //struct defining everything contained within a given move, using the type enum to define the type of the move
    pub name: String,
    pub power: i32,
    pub accuracy: i32,
    pub kind: MoveKind,
    pub m_type: Type,
}

impl Move {
    //initializes the Move slam
    pub fn slam() -> Self {
        Self { //contructor for all of slam's info
            name: String::from("Slam"),
            power: 15,
            accuracy: 80,
            kind: MoveKind::Damage,
            m_type: Type::Earth, 
        }
    }

    //initializes the move flail
    pub fn flail() -> Self {
        Self { //contructor for all of flail's info
            name: String::from("Flail"),
            power: 0,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Earth, 
        }
    } 
    
    //initializes the move wave
    pub fn wave() -> Self {
        Self { //contructor for all of wave's info
            name: String::from("Wave"),
            power: 15,
            accuracy: 80,
            kind: MoveKind::Damage,
            m_type: Type::Water, 
        }
    }

    //initializes the move slice
    pub fn slice() -> Self {
        Self {
            name: String::from("Slice"),
            power: 5,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Water,
        }
    }

    //initializes the move screech
    pub fn screech() -> Self{
        Self {
            name: String::from("Screech"),
            power: 10,
            accuracy: 90,
            kind: MoveKind::Damage,
            m_type: Type::Fire,
        }
    }

    //initializes the move peck
    pub fn peck() -> Self{
        Self{
            name: String::from("Peck"),
            power: 7,
            accuracy: 85,
            kind: MoveKind::Damage,
            m_type: Type::Fire,
        }
    }

    //all moves below are the same

    //initializes the move reincarnate
    pub fn reincarnate() -> Self {
        Self { //contructor for all of reincarnate's info
            name: String::from("Reincarnate"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Time, 
        }
    }

    //initializes the move high tide
    pub fn high_tide() -> Self {
        Self { //contructor for all of high tide's info
            name: String::from("High Tide"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Water, 
        }
    }

    //initializes the move vortex
    pub fn vortex() -> Self {
        Self { //contructor for all of vortex's info
            name: String::from("Vortex"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Healing,
            m_type: Type::Astral, 
        }
    }

    //initializes the move bash
    pub fn bash() -> Self {
        Self { //contructor for all of bash's info
            name: String::from("Bash"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Dark, 
        }
    }

    //initializes the move ram
    pub fn ram() -> Self {
        Self { //contructor for all of ram's info
            name: String::from("Ram"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Wind, 
        }
    }

    //initializes the move wisp
    pub fn wisp() -> Self {
        Self { //contructor for all of wisp's info
            name: String::from("Wisp"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Fire, 
        }
    }

    //initializes the move blitz
    pub fn blitz() -> Self {
        Self { //contructor for all of blitz's info
            name: String::from("Blitz"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Fire, 
        }
    }

    //initializes the move scratch
    pub fn scratch() -> Self {
        Self { //contructor for all of scratch's info
            name: String::from("Scratch"),
            power: 10,
            accuracy: 100,
            kind: MoveKind::Damage,
            m_type: Type::Zen, 
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub owner: bool, //true if owned by player, false if owned by opponent
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
            owner: true,
            texture: String::from("assets/strays/palliub.png")
        }
    }
}

impl Stray {
    //constructors for all individual strays in game
    pub fn bitaxum(owner: bool) -> Self {
        Self { //contructor for all of bitaxum's info
            species: String::from("Bitaxum"),
            s_type: Type::Earth,
            moves: [Some(Move::slam()), Some(Move::flail()), None, None],
            hp: 20,
            atk: 20,
            def: 40,
            spd: 10,
            cur_hp: 20,
            owner,
            texture: String::from("assets/strays/bitaxum.png")
        }
    }

    pub fn palliub(owner: bool) -> Self {
        Self { //contructor for all of palliub's info
            species: String::from("Palliub"),
            s_type: Type::Water,
            moves: [Some(Move::wave()), Some(Move::slice()), Some(Move::flail()), None],
            hp: 15,
            atk: 30,
            def: 20,
            spd: 35,
            cur_hp: 15,
            owner,
            texture: String::from("assets/strays/palliub.png")
        }
    }

    pub fn rubridum(owner: bool) -> Self {
        Self { //contructor for all of rubridum's info
            species: String::from("Rubridum"),
            s_type: Type::Fire,
            moves: [Some(Move::screech()), Some(Move::peck()), None, None],
            hp: 15,
            atk: 35,
            def: 25,
            spd: 25,
            cur_hp: 15,
            owner,
            texture: String::from("assets/strays/rubridum.png")
        }
    }

    pub fn aeternisc(owner: bool) -> Self {
        Self { //contructor for all of aeternisc's info
            species: String::from("Aeternisc"),
            s_type: Type::Time,
            moves: [Some(Move::reincarnate()), Some(Move::flail()), None, None],
            hp: 50,
            atk: 25,
            def: 35,
            spd: 10,
            cur_hp: 50,
            owner,
            texture: String::from("assets/strays/aeternisc.png")
        }
    }

    pub fn solikigoi(owner: bool) -> Self {
        Self { //contructor for all of solikigoi's info
            species: String::from("Solikigoi"),
            s_type: Type::Water,
            moves: [Some(Move::wave()), Some(Move::high_tide()), None, None],
            hp: 20,
            atk: 15,
            def: 35,
            spd: 15,
            cur_hp: 20,
            owner,
            texture: String::from("assets/strays/solikigoi.png")
        }
    }

    pub fn catis(owner: bool) -> Self {
        Self { //contructor for all of catis's info
            species: String::from("Catis"),
            s_type: Type::Earth,
            moves: [Some(Move::vortex()), Some(Move::bash()), Some(Move::flail()), None],
            hp: 25,
            atk: 15,
            def: 30,
            spd: 10,
            cur_hp: 25,
            owner,
            texture: String::from("assets/strays/catis.png")
        }
    }
    
    pub fn cespae(owner: bool) -> Self {
        Self { //constructor for all of cespae's info 
            species: String::from("Cespae"),
            s_type: Type::Zen,
            moves: [Some(Move::ram()), Some(Move::bash()), Some(Move::flail()), None],
            hp: 15,
            atk: 20,
            def: 30,
            spd: 20,
            cur_hp: 15,
            owner,
            texture: String::from("assets/strays/cespae.png")
        }
    }

    pub fn omikae(owner: bool) -> Self {
        Self { //constructor for all of omikae's info 
            species: String::from("Omikae"),
            s_type: Type::Astral,
            moves: [Some(Move::wisp()), Some(Move::blitz()), None, None],
            hp: 20,
            atk: 15,
            def: 30,
            spd: 20,
            cur_hp: 20,
            owner,
            texture: String::from("assets/strays/omikae.png")
        }
    }

    pub fn carerus(owner: bool) -> Self {
        Self { //constructor for all of carerus's info 
            species: String::from("Carerus"),
            s_type: Type::Dark,
            moves: [Some(Move::scratch()), Some(Move::slice()), None, None],
            hp: 20,
            atk: 30,
            def: 15,
            spd: 20,
            cur_hp: 20,
            owner,
            texture: String::from("assets/strays/carerus.png")
        }
    }
}


