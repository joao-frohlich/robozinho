use crate::factory::Factory;
use crate::terrain::Terrain;
use crate::tool::ToolType;
use bevy::prelude::Component;

#[derive(Clone, Copy, Component)]
pub struct Cell {
    pub terrain: Terrain,
    pub tool: Option<ToolType>,
    pub factory: Option<Factory>,
}

impl Cell {
    pub fn new(terrain: Terrain, tool: Option<ToolType>, factory: Option<Factory>) -> Self {
        Self {
            terrain,
            tool,
            factory,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(Terrain::Grass, None, None)
    }
}
