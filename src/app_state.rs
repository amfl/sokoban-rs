use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Result, Value};

#[derive(Clone, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
    Target,
    Crate,
    CrateOnTarget,
    Player,
    PlayerOnTarget,
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

    pub fn get2(&self, (x, y): (i16, i16)) -> &Tile {
        self.get(x, y)
    }

    pub fn get_mut(&mut self, x: i16, y: i16) -> &mut Tile {
        let k = y*self.w + x;
        &mut self.data[k as usize]
    }

    pub fn set(&mut self, (x, y): (i16, i16), t: Tile) {
        let k = y*self.w + x;
        self.data[k as usize] = t;
    }

    // Returns true if the player actually moved
    pub fn player_move(&mut self, dx: i16, dy: i16) -> bool {
        // save coords for quick access
        // let px = self.player_x;
        // let py = self.player_y;
        let coord_player = (self.player_x,      self.player_y);
        let coord_dest   = (self.player_x+dx,   self.player_y+dy);
        let coord_target = (self.player_x+2*dx, self.player_y+2*dy);

        let dest = self.get2(coord_dest);
        let target = self.get2(coord_target);

        if *dest == Tile::Floor || *dest == Tile::Target {
            // Woo, move
        } else if (*dest == Tile::Crate && *target == Tile::Floor) {
            self.set(coord_dest, Tile::Floor);
            self.set(coord_target, Tile::Crate);
        } else if (*dest == Tile::Crate && *target == Tile::Target) {
            self.set(coord_dest, Tile::Floor);
            self.set(coord_target, Tile::CrateOnTarget);
        } else if (*dest == Tile::CrateOnTarget && *target == Tile::Floor) {
            self.set(coord_dest, Tile::Target);
            self.set(coord_target, Tile::Crate);
        } else if (*dest == Tile::CrateOnTarget && *target == Tile::Target) {
            self.set(coord_dest, Tile::Target);
            self.set(coord_target, Tile::CrateOnTarget);
        } else {
            return false
        }

        self.player_x += dx;
        self.player_y += dy;
        true
    }

    pub fn load_level(&mut self, filename: &str, level: usize) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        // Parse the string of data into serde_json::Value.
        let v: Vec<Vec<Vec<u8>>> = serde_json::from_str(&data)?;

        let level_data = &v[level];

        let mut new_level = Vec::<Tile>::default();

        self.w = level_data[0].len() as i16;
        self.h = level_data.len() as i16;
        self.player_x = self.w / 2;
        self.player_y = self.h / 2;
        self.view_center_x = self.w / 2;
        self.view_center_y = self.h / 2;

        for (y, row) in level_data.iter().enumerate() {
           for (x, cell) in row.iter().enumerate() {
                new_level.push(match cell {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Floor,
                    3 => Tile::Target,
                    4 => Tile::Crate,
                    5 => Tile::CrateOnTarget,
                    6 => {
                        self.player_x = x as i16;
                        self.player_y = y as i16;
                        Tile::Floor
                    }
                    7 => {
                        self.player_x = x as i16;
                        self.player_y = y as i16;
                        Tile::Target
                    }
                    _ => Tile::Empty,
                });
           }
        }

        self.data = new_level;

        Ok(())
    }
}
