mod monsters;

pub(crate) struct Order {
    list : vec<Monster>
}

impl Order {
    pub fn new(list: vec<Monster>) {
        Order {
            list,
        }
    }
}