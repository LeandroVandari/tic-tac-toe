use crate::Player;

pub trait Cell {
    fn owner(&self) -> Option<&Player>;
}