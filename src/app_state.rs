use rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum Tile {
    Wall,
    Crate,
    Player,
    Floor,
}
pub struct AppState {
    pub w: i16,
    pub h: i16,
    pub view_center_x: i16,
    pub view_center_y: i16,
    data: Vec<Tile>,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState::new(16,16)
    }
}
impl AppState {
    pub fn new(w: i16, h: i16) -> AppState {
        let mut v = Vec::default();

        // Populate with entirely random data
        let mut rng = thread_rng();
        for i in 0..(w*h) {
            let t = match rng.gen_range(0, 3) {
                0 => Tile::Wall,
                1 => Tile::Crate,
                _ => Tile::Floor,
            };
            v.push(t);
        }
        AppState{
            w: w,
            h: h,
            view_center_x: w/2,
            view_center_y: h/2,
            // data: vec![Tile::Floor; (w*h) as usize],
            data: v,
        }
    }
    pub fn get(&self, x: i16, y: i16) -> &Tile {
        let k = y*self.w + x;
        &self.data[k as usize]
    }
}
