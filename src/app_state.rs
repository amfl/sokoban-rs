pub struct AppState {
    pub x: u16,
    pub y: u16,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState{ x: 0, y: 0 }
    }
}
