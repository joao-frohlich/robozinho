use crate::terrain::Terrain;
use crate::tool::Tool;
use bevy::prelude::Component;

#[derive(Clone, Copy, Component)]
pub struct Cell {
    pub item: Option<Tool>,
    pub terrain: Terrain,
}

impl Cell {
    pub fn new(item: Option<Tool>, terrain: Terrain) -> Self {
        Self { item, terrain }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(None, Terrain::Grass)
    }
}
