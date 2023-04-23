/****************************************************/
// Created by: Logan Schmalz
// Description: Definition of storable items and how they are stored in the bag
/****************************************************/
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Item {
    Berry,
}

pub type ItemList = Vec<(Item, u32)>;

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Berry => write!(f, "Berry"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Bag {
    pub items: ItemList,
}

impl Bag {
    pub fn new() -> Bag {
        Bag { items: vec![] }
    }

    //this function looks through the list of items already in the bag and if it is already there it adds the new items and if not it adds a new instance to the list
    pub fn add_item(&mut self, item: Item, amount: u32) -> bool {
        let i = self.items.iter_mut().position(|i| i.0 == item);
        match i {
            Some(i) => {
                self.items[i] = (item, self.items[i].1 + amount);
            }
            _ => self.items.push((item, 1)),
        }
        true
    }
}
