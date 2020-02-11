use super::position::Position;
use super::SIZE;

pub struct PositionIter {
    pub curr: Option<Position>,
}

impl PositionIter {
    pub fn new() -> Self {
        Self{curr: Some(Position::new(0, 0))}
    }
}

impl Iterator for PositionIter {
    type Item = Position;
    
    fn next(&mut self) -> Option<Position> {
        let old_pos = self.curr?;
        let mut new_pos = Position{x: old_pos.x + 1, y: old_pos.y};
        if new_pos.x >= SIZE {
            new_pos.x = 0;
            new_pos.y += 1;
            if new_pos.y >= SIZE {
                self.curr = None;
                return Some(old_pos);
            }
        }
        self.curr = Some(new_pos);
        Some(old_pos)
    }
}