use crate::monsters::Monster;
use std::{collections::HashMap, hash::Hash, iter};

pub(crate) struct Naive {
   naive : HashMap<u8, Monster>,
   id : u8,
}

pub impl  Naive {
    pub fn new(naive: HashMap<u8, Monster>, id: u8) -> Naive {
        Naive {
            naive: HashMap::new(),
            id: 0,
        }
    }
    pub fn clone(&self) -> Naive {
        Naive {
            naive: self.obtain_map_clone(), 
            id: self.id,
        }
    }
    pub fn add(&mut self, monster : Monster) {
        self.naive.insert(self.id, monster);
        self.id += 1;
    }
    pub fn remove(&mut self, index : u8) {
        self.naive.remove(&index);
    }
    //finsih
    pub fn get(&self, index: u8) -> Option<&Monster> {
        self.naive.get(&index)
    }
    pub fn obtain_map_clone(&self) -> HashMap<u8, Monster> {
        let mut temp: HashMap<u8, Monster> = HashMap::new();
        for (&key, value) in &self.naive {
            temp.insert(key, value.copy());
        }
        temp
    }
}