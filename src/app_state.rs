use rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum Tile {
    Wall,
    Crate,
    Player,
    Floor,
}
pub struct AppState {
    pub w: u16,
    pub h: u16,
    pub view_center_x: u16,
    pub view_center_y: u16,
    data: Vec<Tile>,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState::new(16,16)
    }
}
impl AppState {
    pub fn new(w: u16, h: u16) -> AppState {
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
    pub fn get(&self, x: u16, y: u16) -> &Tile {
        let k = y*self.w + x;
        &self.data[k as usize]
    }
}
