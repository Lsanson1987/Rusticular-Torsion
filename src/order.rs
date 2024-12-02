use crate::monsters::Monster;

pub(crate) struct Order {
    list : Vec<Monster>
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
    pub fn sort(&mut self) {

    }
    //create a getter for a value

    //think of some way to get the index of the list like I'm not really sure how yo yet....
}