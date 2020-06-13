use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq)]
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
    pub player_x: i16,
    pub player_y: i16,
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
            player_x: w/2,
            player_y: h/2,
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

    pub fn get_mut(&mut self, x: i16, y: i16) -> &mut Tile {
        let k = y*self.w + x;
        &mut self.data[k as usize]
    }

    // Returns true if the player actually moved
    pub fn player_move(&mut self, dx: i16, dy: i16) -> bool {
        let px = self.player_x;
        let py = self.player_y;
        let dest = self.get(px + dx, py + dy);
        let target = self.get(px + 2*dx, py + 2*dy);

        if *dest == Tile::Floor ||
           (*dest == Tile::Crate && *target == Tile::Floor)
        {
            if *dest == Tile::Crate {
                {
                    let mut d = self.get_mut(px + dx, py + dy);
                    *d = Tile::Floor;
                }
                {
                    let mut t = self.get_mut(px + 2*dx, py + 2*dy);
                    *t = Tile::Crate;
                }
            }
            self.player_x += dx;
            self.player_y += dy;
        }

        true
    }
}
