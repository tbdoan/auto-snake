use rand::Rng;

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub food: Coord,
}

impl Arena {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            food: spawn_food(width, height),
        }
    }

    pub fn regen_food(&mut self) {
        self.food = spawn_food(self.width, self.height);
    }
}

fn spawn_food(width: u32, height: u32) -> Coord {
    Coord {
        x: rand::thread_rng().gen_range(0..width),
        y: rand::thread_rng().gen_range(0..height),
    }
}
