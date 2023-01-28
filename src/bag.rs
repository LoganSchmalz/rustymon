#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Item {
    Berry,
}

#[derive(Debug)]
pub struct Bag {
    items: Vec<(Item, u32)>,
}

impl Bag {
    pub fn new() -> Bag {
        Bag { items: vec![] }
    }

    pub fn add_one(&mut self, item: Item) {
        let i = self.items.iter().position(|i| i.0 == item);
        match i {
            Some(i) => {
                self.items[i] = (item, self.items[i].1 + 1);
            }
            _ => self.items.push((item, 1)),
        }
    }

    pub fn add_multiple(&mut self, item: Item, amount: u32) {
        let i = self.items.iter_mut().position(|i| i.0 == item);
        match i {
            Some(i) => {
                self.items[i] = (item, self.items[i].1 + amount);
            }
            _ => self.items.push((item, 1)),
        }
    }
}
