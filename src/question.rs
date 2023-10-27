pub struct Question {
    pub real_elo: u32,
    pub estimated_elo: u32,
    pub age: usize,
}

impl Question {
    pub fn new(elo: u32) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 1600,
            age: 0,
        }
    }
}
