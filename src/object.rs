use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ObjectType {
    Tree,
    Rock,
    Bush,
}

impl ObjectType {
    pub fn atlas_index(&self) -> u32 {
        match self {
            ObjectType::Tree => 0,
            ObjectType::Rock => 1,
            ObjectType::Bush => 2,
        }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, ObjectType::Tree)
    }
}

pub struct WorldObject {
    pub position: Vec2,
    pub object_type: ObjectType,
}
