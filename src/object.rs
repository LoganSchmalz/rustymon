use crate::coordinate::Coordinate;
use crate::menu::{self, MenuManager};
use crate::{render, tilemap};

use std::collections::HashMap;
//use num_derive::FromPrimitive;
//use num_traits::FromPrimitive;
use std::{fs, path::Path};

mod berry;
mod door;
pub mod npc;

use self::berry::Berry;
use self::door::Door;
use self::npc::{Character, NPC};

#[enum_delegate::register]
pub trait TObject {
    fn get_pos(&self) -> Coordinate;
    fn set_pos(&mut self, pos: Coordinate);
    fn get_prev_pos(&self) -> Coordinate {
        self.get_pos()
    }
    fn interact(
        &mut self,
        renderer: &mut render::Renderer,
        menu_man: &mut MenuManager,
        player_position: Coordinate,
    ) -> bool; //returns if obj should be removed from map
    fn update(
        &mut self,
        _delta_time: &f64,
        _map: &tilemap::TileMap,
        _collision_manager: &CollisionManager,
    ) -> bool {
        false
    } //returns if obj actually updated
}

#[enum_delegate::implement(TObject)]
pub enum Object {
    //None, //0
    Berry(Berry), //1
    Door(Door),   //2
    NPC(NPC),     /*
                  Dad(NPC), //3
                  Jodo(NPC), //4
                  Sika(NPC), //5
                  */
}

pub struct CollisionManager {
    collisions: HashMap<usize, bool>, //the u32 is derived from the coordinate -> u32 calculation, consider replacing this with some sort of direct hashing in the future
}

impl CollisionManager {
    pub fn check_collision(&self, pos: Coordinate, prev_pos: Coordinate, size_x: usize) -> bool {
        if pos == prev_pos {
            return false;
        }
        let Some(collision) = self.collisions.get(&pos.to_usize(size_x)) else { return false; };
        *collision
    }
}

pub struct ObjectManager {
    pub objects: Vec<Object>,
    pub collision_manager: CollisionManager,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        let objects: Vec<Object> = vec![];
        ObjectManager {
            objects,
            collision_manager: CollisionManager {
                collisions: HashMap::new(),
            },
        }
    }

    pub fn load_objects(&mut self, mapfolder: &Path) {
        self.objects.clear();
        self.collision_manager.collisions.clear();

        let dim: Vec<usize> = fs::read_to_string(mapfolder.join("dim.txt"))
            .expect(&format!("{}dim.txt not found", mapfolder.to_str().unwrap()))
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("Not an integer!"))
            .collect();

        if dim.len() != 2 {
            panic!(
                "{}dim.txt does not contain exactly 2 values",
                mapfolder.to_str().unwrap()
            )
        }
        let size_x = dim.get(0).unwrap();

        let objects: Vec<u32> = fs::read_to_string(mapfolder.join("objects.txt"))
            .expect(&format!(
                "{}objects.txt not found",
                mapfolder.to_str().unwrap()
            ))
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Not an integer!"))
            .collect();

        //println!("{:?}", objects);

        //todo: improve object loading, stop loading from map by coordinates and instead load from list
        for (idx, obj) in objects.iter().enumerate() {
            let pos = Coordinate((idx % size_x) as f64, (idx / size_x) as f64);

            match obj {
                1 => {
                    self.objects.push(Object::Berry(Berry::new(pos)));
                }
                2 => {
                    self.objects
                        .push(Object::Door(Door::new(pos, (0, Coordinate(2.0, 1.0)))));
                }
                3 => {
                    self.objects
                        .push(Object::NPC(NPC::new(pos, Character::Dad, None)));
                }
                4 => {
                    self.objects.push(Object::NPC(NPC::new(
                        pos,
                        Character::Jodo,
                        Some(Coordinate(pos.0 - 1.0, pos.1)),
                    )));
                }
                5 => {
                    self.objects
                        .push(Object::NPC(NPC::new(pos, Character::Sika, None)));
                }
                _ => {
                    continue;
                }
            }
            self.collision_manager
                .collisions
                .insert(pos.to_usize(dim[0]), true);
        }
    }

    pub fn update_objects(&mut self, delta_time: &f64, map: &tilemap::TileMap) {   
        //consider an alternative loop so collision management is not done without the object manager??
        //https://stackoverflow.com/questions/71302444/borrow-a-vector-inside-a-loop
        for obj in self.objects.iter_mut() {
            let recompute_collision = obj.update(delta_time, map, &self.collision_manager);
            if recompute_collision {
                let prev_pos = obj.get_prev_pos();
                let new_pos = obj.get_pos();

                if new_pos.dist(prev_pos) >= 1.0 {
                    self.collision_manager
                        .collisions
                        .insert(obj.get_prev_pos().to_usize(map.size_x), false);
                }

                self.collision_manager
                    .collisions
                    .insert(new_pos.to_usize(map.size_x), true);
            }
        }
    }

    pub fn interact(
        &mut self,
        pos: Coordinate,
        player_position: Coordinate,
        renderer: &mut render::Renderer,
        menu_man: &mut menu::MenuManager,
        map: &tilemap::TileMap,
    ) {
        match self.get_obj(pos) {
            Some(idx) => {
                //todo: change this to use new collision checking
                if self.objects[idx].interact(renderer, menu_man, player_position) {
                    self.collision_manager
                        .collisions
                        .insert(self.objects[idx].get_prev_pos().to_usize(map.size_x), false);
                    self.objects.remove(idx);
                }
            }
            _ => {}
        }
    }

    pub fn get_obj(&self, pos: Coordinate) -> Option<usize> {
        for (idx, obj) in self.objects.iter().enumerate() {
            if pos == obj.get_pos() {
                return Some(idx);
            }
        }
        None
    }
}
