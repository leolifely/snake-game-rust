#[derive(Copy, Clone)]
pub struct Fruit {
    pub coords: [i32; 2],
}

impl Fruit {
    pub fn new(coords: [i32; 2]) -> Fruit{
        Fruit {coords}
    }
    
    pub fn move_to_random_location(&mut self, screen_size: [i32; 2]) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.coords = [rng.gen_range(0..(screen_size[0] / 25)) * 25, rng.gen_range(0..(screen_size[1] / 25)) * 25];
    }
}