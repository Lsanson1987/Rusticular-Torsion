use rand::Rng;


pub fn rollDice(num_dice: u32, sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let mut total = 0;
    for _ in 0..num_dice {
        let roll = rng.gen_range(1..=sides);
        total += roll;
    }
    total
}