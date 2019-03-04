extern crate rand;

use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;


pub struct Grid {
    pub size: u32,
    pub cmap: HashMap<u32, [f32; 4]>,
}

impl Grid {
    pub fn new() -> Grid {
        let size = 8;
        let mut rng = thread_rng();
        let mut cmap: HashMap<u32, [f32; 4]> = HashMap::new();
        for i in 0..(size * size) {
            let v: [f32; 4] = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
            cmap.insert(i, v);
        }
        Grid { size, cmap}
    }
}