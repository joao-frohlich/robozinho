pub struct Path {
    pub moves: Vec<(i32, i32)>,
}

impl Path {
    pub fn new() -> Self {
        Self { moves: vec![] }
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}
