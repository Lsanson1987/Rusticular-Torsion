
pub struct Monster {
    name: String,
    initiative: u8,
    hit_points: u8,
    armor_class: u8,
    notes: String,
}

impl Monster {
    pub fn new(name: &str, initiative: u8, hit_points: u8, armor_class: u8, notes: &str) -> Monster {
      Monster {  name: name.to_string(),
        initiative,
        hit_points,
        armor_class,
        notes: notes.to_string(),
      }
    }   
    pub fn copy(&self) -> Monster {
        Monster {
            name: self.name.clone(),
            initiative: self.initiative,
            hit_points: self.hit_points,
            armor_class: self.armor_class,
            notes: self.notes.clone(),
        }
    }
    pub fn display_name(&self) {
        println!("{}", self.name);
    }
    pub fn display_initiative(&self) {
        println!("{}", self.initiative);
    }
    pub fn display_hit_points(&self) {
        println!("{}", self.hit_points);
    }
    pub fn display_armor_class(&self) {
        println!("{}", self.armor_class);
    }
    pub fn display_notes(&self) {
        println!("{}", self.notes);
    }
    pub fn change_initiative(&mut self, int: u8) {
        self.initiative = int;
    }
    pub fn change_armor(&mut self, int: u8) {
        self.armor_class = int;
    }
    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn change_note(&mut self, name: &str) {
        self.notes = name.to_string();
    }
    pub fn damage(&mut self, x: u8) {
        self.hit_points = self.hit_points - x;
    }
    pub fn heal(&mut self, x: u8) {
        self.hit_points = self.hit_points + x;
    }
}