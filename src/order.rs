use crate::monsters::Monster;

pub(crate) struct Order {
    list : Vec<Monster>,
}

impl Order {
    pub fn new(list: Vec<Monster>) -> Order{
        Order {
            list,
        }
    }
    pub fn new_row(&mut self) {
        let temp = Monster::new("", 0, 0, 0, "");
        self.list.push(temp);
    }
    pub fn remove_row(&mut self, index: u8) {
        self.list.remove(index.into());
    }
    //compete this sorting try and use quick sort
    // pub fn sort(&mut self) {

    // }
    pub fn getMonster(&mut self, index: usize) -> Monster {
        return self.list[index].copy()
    }
}