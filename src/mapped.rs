use crate::monsters::Monster;
use std::collections::HashMap;

pub(crate) struct Naive {
   naive : HashMap<u8, Monster>,
   id : u8,
}

impl  Naive {
    pub fn new(naive: HashMap<u8, Monster>, id: u8) -> Naive {
        Naive {
            naive,
            id,
        }
    }
    pub fn add(&mut self, monster : Monster) {
        self.naive.insert(self.id, monster);
        self.id += 1;
    }
    pub fn remove(mut self, index : u8) {
        self.naive.remove(&index);
    }
    //finsih
    pub  fn get(&mut self, index : u8) -> Monster {
        self.naive[&index]
    }
}